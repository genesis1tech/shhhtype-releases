use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

/// macOS window level for the overlay panel — NSPopUpMenuWindowLevel (101).
/// This ensures the overlay renders above all normal windows, floating panels,
/// and modal dialogs. Combined with NSPanel swizzle, it also appears over
/// full-screen Spaces.
#[cfg(target_os = "macos")]
pub const OVERLAY_WINDOW_LEVEL: i64 = 101;

/// Swizzle a Tauri-created NSWindow into an NSPanel at runtime.
///
/// NSPanel is the only window type macOS allows to appear over full-screen apps.
/// This performs an ISA swizzle (object_setClass) to convert the underlying
/// NSWindow to NSPanel, then sets the NonactivatingPanel style mask so the
/// overlay never steals focus from the user's app.
#[cfg(target_os = "macos")]
pub unsafe fn swizzle_to_nspanel(ns_window: cocoa::base::id) {
    use cocoa::appkit::{NSWindow, NSWindowCollectionBehavior};
    use objc::runtime::Class;
    use objc::{msg_send, sel, sel_impl};

    extern "C" {
        fn object_setClass(
            obj: *mut objc::runtime::Object,
            cls: *const Class,
        ) -> *mut objc::runtime::Object;
    }

    // Swizzle the window's ISA to NSPanel (a subclass of NSWindow)
    let nspanel_class = Class::get("NSPanel").expect("NSPanel class not found");
    object_setClass(ns_window as *mut objc::runtime::Object, nspanel_class);

    // Add NonactivatingPanel style mask (1 << 7 = 128) to prevent focus stealing
    let current_mask: u64 = msg_send![ns_window, styleMask];
    let non_activating: u64 = 1 << 7; // NSWindowStyleMaskNonactivatingPanel
    let _: () = msg_send![ns_window, setStyleMask: current_mask | non_activating];

    // Sync WindowServer tags after style mask change (private but stable API)
    let _: () = msg_send![ns_window, _setPreventsActivation: true];

    // Collection behaviors: appear in all Spaces including full-screen
    ns_window.setCollectionBehavior_(
        NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
            | NSWindowCollectionBehavior::NSWindowCollectionBehaviorStationary
            | NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary
            | NSWindowCollectionBehavior::NSWindowCollectionBehaviorIgnoresCycle,
    );

    // Set window level
    ns_window.setLevel_(OVERLAY_WINDOW_LEVEL);

    // Remove window shadow for clean transparent look
    let _: () = msg_send![ns_window, setHasShadow: false];

    log::info!("Overlay NSWindow swizzled to NSPanel (level {})", OVERLAY_WINDOW_LEVEL);
}

/// Show the overlay window and ensure it stays above all other windows.
///
/// Re-applies the macOS window level and forces the window to the front on
/// every show because macOS may reset window ordering after hide/show cycles.
pub fn show_overlay(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("overlay") {
        let _ = w.show();

        #[cfg(target_os = "macos")]
        {
            use cocoa::appkit::NSWindow;
            use cocoa::base::id;
            if let Ok(ns_ptr) = w.ns_window() {
                let ns_window: id = ns_ptr as id;
                unsafe {
                    ns_window.setLevel_(OVERLAY_WINDOW_LEVEL);
                    ns_window.orderFrontRegardless();
                }
            }
        }
    }
}

/// Hide the overlay window.
pub fn hide_overlay(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("overlay") {
        let _ = w.set_ignore_cursor_events(true);
        let _ = w.hide();
    }
}

/// Enable cursor events on overlay so user can click "Rewrite?" button.
pub fn enable_overlay_interaction(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("overlay") {
        let _ = w.set_ignore_cursor_events(false);
    }
}

/// Disable cursor events on overlay (click-through mode).
pub fn disable_overlay_interaction(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("overlay") {
        let _ = w.set_ignore_cursor_events(true);
    }
}

/// Toggle the settings window (create on first use).
pub fn toggle_settings(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("settings") {
        if w.is_visible().unwrap_or(false) {
            let _ = w.hide();
        } else {
            let _ = w.show();
            let _ = w.set_focus();
        }
    } else {
        let _ = WebviewWindowBuilder::new(
            app,
            "settings",
            WebviewUrl::App("index.html".into()),
        )
        .title("vox2txt Settings")
        .inner_size(700.0, 500.0)
        .resizable(true)
        .center()
        .build();
    }
}

/// Show the welcome/onboarding window.
pub fn show_welcome(app: &AppHandle) {
    if app.get_webview_window("welcome").is_some() {
        return;
    }
    let _ = WebviewWindowBuilder::new(
        app,
        "welcome",
        WebviewUrl::App("index.html".into()),
    )
    .title("Welcome to vox2txt")
    .inner_size(600.0, 520.0)
    .resizable(false)
    .center()
    .build();
}
