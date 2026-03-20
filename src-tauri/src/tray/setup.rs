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

    TrayIconBuilder::new()
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
