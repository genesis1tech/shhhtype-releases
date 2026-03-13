use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

/// Show the overlay window.
pub fn show_overlay(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("overlay") {
        let _ = w.show();
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
