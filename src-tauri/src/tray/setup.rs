use anyhow::Result;
use crate::state::AppState;
use std::sync::Arc;
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    App, Emitter, Manager,
};

/// Create the system tray icon with context menu.
pub fn create_tray(app: &App) -> Result<()> {
    let settings_item = MenuItem::with_id(app, "settings", "Settings...", true, None::<&str>)?;
    let clear_comp_item = MenuItem::with_id(app, "clear_composition", "Clear Composition", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit ShhhType", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&settings_item, &clear_comp_item, &separator, &quit_item])?;

    TrayIconBuilder::new()
        .icon(tauri::include_image!("icons/32x32.png"))
        .menu(&menu)
        .tooltip("ShhhType")
        .on_menu_event(|app, event| match event.id.as_ref() {
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
            _ => {}
        })
        .build(app)?;

    log::info!("System tray created");
    Ok(())
}
