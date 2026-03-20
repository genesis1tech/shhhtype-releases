use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait};
use crate::state::AppState;
use std::sync::Arc;
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem, Submenu, CheckMenuItem},
    tray::TrayIconBuilder,
    App, Emitter, Manager,
};

/// Build the audio input device submenu with the current selection checked.
fn build_audio_submenu(app: &App, selected: &Option<String>) -> Result<Submenu<tauri::Wry>> {
    let host = cpal::default_host();
    let default_name = host.default_input_device().and_then(|d| d.name().ok());

    let default_label = format!(
        "System Default{}",
        default_name.as_ref().map(|n| format!(" ({})", n)).unwrap_or_default()
    );
    let default_checked = selected.is_none();
    let default_item = CheckMenuItem::with_id(
        app,
        "audio_device__default",
        &default_label,
        true,
        default_checked,
        None::<&str>,
    )?;

    let submenu = Submenu::with_id(app, "audio_input", "Audio Input", true)?;
    submenu.append(&default_item)?;

    if let Ok(devices) = host.input_devices() {
        for device in devices {
            if let Ok(name) = device.name() {
                let id = format!("audio_device__{}", name);
                let checked = selected.as_deref() == Some(&name);
                let item = CheckMenuItem::with_id(app, &id, &name, true, checked, None::<&str>)?;
                submenu.append(&item)?;
            }
        }
    }

    Ok(submenu)
}

/// Update the tray title to show segment count as a small red number.
/// Shows nothing when count is 0.
pub fn update_tray_segment_count(app: &tauri::AppHandle, count: usize) {
    if let Some(tray) = app.tray_by_id("main") {
        if count > 0 {
            let title = format!("{}", count);
            // Set plain title first (required by muda internals)
            let _ = tray.set_title(Some(&title));

            // Style it: red color, small font (50% of default ~13pt → ~7pt)
            #[cfg(target_os = "macos")]
            style_tray_title_red(&title);
        } else {
            let _ = tray.set_title(None::<&str>);
        }
    }
}

/// Dispatch styled tray title update to the main thread.
/// Finds the NSStatusBarButton and sets an attributedTitle with red color + small font.
#[cfg(target_os = "macos")]
fn style_tray_title_red(title: &str) {
    // We need to box the title string and pass it as context to GCD
    let title_owned = title.to_string();
    let boxed = Box::into_raw(Box::new(title_owned));

    extern "C" fn do_style(ctx: *mut std::ffi::c_void) {
        unsafe {
            let title = Box::from_raw(ctx as *mut String);
            style_tray_title_red_main_thread(&title);
        }
    }

    unsafe {
        extern "C" {
            static _dispatch_main_q: u8;
            fn dispatch_async_f(
                queue: *const u8,
                context: *mut std::ffi::c_void,
                work: extern "C" fn(*mut std::ffi::c_void),
            );
        }
        dispatch_async_f(
            std::ptr::addr_of!(_dispatch_main_q),
            boxed as *mut std::ffi::c_void,
            do_style,
        );
    }
}

/// Must be called on the main thread. Finds the NSStatusBarButton and sets
/// an attributed title with red color and small font.
#[cfg(target_os = "macos")]
unsafe fn style_tray_title_red_main_thread(title: &str) {
    use cocoa::base::{id, nil};
    use cocoa::foundation::NSString;
    use objc::runtime::Class;
    use objc::{msg_send, sel, sel_impl};

    // Build attributes: red color, tiny font, raised baseline
    let font: id = msg_send![
        Class::get("NSFont").unwrap(),
        monospacedDigitSystemFontOfSize: 3.5_f64
        weight: 0.4_f64  // NSFontWeightSemibold for legibility at small size
    ];
    let red: id = msg_send![Class::get("NSColor").unwrap(), systemRedColor];

    // Positive baseline offset moves text upward
    let baseline_offset: id = msg_send![Class::get("NSNumber").unwrap(), numberWithDouble: 4.0_f64];

    let font_key = NSString::alloc(nil).init_str("NSFont");
    let color_key = NSString::alloc(nil).init_str("NSColor");
    let baseline_key = NSString::alloc(nil).init_str("NSBaselineOffset");
    let keys = [font_key, color_key, baseline_key];
    let vals = [font, red, baseline_offset];

    let attrs: id = msg_send![
        Class::get("NSDictionary").unwrap(),
        dictionaryWithObjects: vals.as_ptr()
        forKeys: keys.as_ptr()
        count: 3_usize
    ];

    let ns_title = NSString::alloc(nil).init_str(title);
    let attr_str: id = msg_send![Class::get("NSAttributedString").unwrap(), alloc];
    let attr_str: id = msg_send![attr_str, initWithString: ns_title attributes: attrs];

    // Find our NSStatusBarButton by iterating app windows.
    // NSStatusItem buttons live inside private NSStatusBarWindow instances.
    let ns_app: id = msg_send![Class::get("NSApplication").unwrap(), sharedApplication];
    let windows: id = msg_send![ns_app, windows];
    let win_count: usize = msg_send![windows, count];

    for i in 0..win_count {
        let window: id = msg_send![windows, objectAtIndex: i];
        let class_name: id = msg_send![window, className];
        if class_name == nil {
            continue;
        }
        let name_cstr: *const std::ffi::c_char = msg_send![class_name, UTF8String];
        if name_cstr.is_null() {
            continue;
        }
        let name = std::ffi::CStr::from_ptr(name_cstr).to_str().unwrap_or("");

        if name == "NSStatusBarWindow" {
            let content_view: id = msg_send![window, contentView];
            if content_view == nil {
                continue;
            }
            // Check if contentView responds to title
            let responds: bool = msg_send![content_view, respondsToSelector: sel!(title)];
            if !responds {
                continue;
            }
            let btn_title: id = msg_send![content_view, title];
            if btn_title == nil {
                continue;
            }
            let btn_cstr: *const std::ffi::c_char = msg_send![btn_title, UTF8String];
            if btn_cstr.is_null() {
                continue;
            }
            let btn_str = std::ffi::CStr::from_ptr(btn_cstr).to_str().unwrap_or("");
            if btn_str == title {
                let _: () = msg_send![content_view, setAttributedTitle: attr_str];
                return;
            }
        }
    }
}

/// Create the system tray icon with context menu.
pub fn create_tray(app: &App) -> Result<()> {
    let state = app.state::<Arc<AppState>>();
    let selected_device = state.config.read().audio_input_device.clone();

    let settings_item = MenuItem::with_id(app, "settings", "Settings...", true, None::<&str>)?;
    let clear_comp_item = MenuItem::with_id(app, "clear_composition", "Clear Composition", true, None::<&str>)?;
    let audio_submenu = build_audio_submenu(app, &selected_device)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit ShhhType", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[
        &settings_item,
        &audio_submenu,
        &clear_comp_item,
        &separator,
        &quit_item,
    ])?;

    TrayIconBuilder::with_id("main")
        .icon(tauri::include_image!("icons/tray-icon@2x.png"))
        .icon_as_template(true)
        .menu(&menu)
        .tooltip("ShhhType")
        .on_menu_event(|app, event| {
            let id = event.id.as_ref();
            match id {
                "settings" => {
                    crate::windows::toggle_settings(app);
                }
                "clear_composition" => {
                    let state = app.state::<Arc<AppState>>();
                    state.composition.lock().clear();
                    update_tray_segment_count(app, 0);
                    let _ = app.emit("composition-cleared", ());
                    log::info!("Composition buffer cleared via tray menu");
                }
                "quit" => {
                    app.exit(0);
                }
                _ if id.starts_with("audio_device__") => {
                    let state = app.state::<Arc<AppState>>();
                    let device_name = if id == "audio_device__default" {
                        None
                    } else {
                        Some(id.strip_prefix("audio_device__").unwrap().to_string())
                    };

                    log::info!("Audio input changed via tray: {:?}", device_name);
                    {
                        let mut config = state.config.write();
                        config.audio_input_device = device_name;
                        if let Err(e) = config.save(&state.data_dir) {
                            log::error!("Failed to save settings: {}", e);
                        }
                    }
                    let _ = app.emit("settings-changed", ());
                }
                _ => {}
            }
        })
        .build(app)?;

    log::info!("System tray created");
    Ok(())
}
