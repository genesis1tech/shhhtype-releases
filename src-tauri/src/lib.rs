mod audio;
mod commands;
mod config;
mod db;
mod hotkey;
mod inject;
mod state;
mod transcribe;
mod tray;
mod vad;
mod windows;

use config::settings::InjectionMethod;
use hotkey::manager::HotkeyMode;
use state::AppState;
use std::sync::Arc;
use tauri::{Emitter, Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    let app_state = AppState::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage(Arc::new(app_state))
        .invoke_handler(tauri::generate_handler![
            commands::start_recording,
            commands::stop_recording,
            commands::cancel_recording,
            commands::get_recording_state,
            commands::get_settings,
            commands::update_settings,
            commands::get_history,
            commands::delete_history_entry,
            commands::get_dictionary,
            commands::update_dictionary,
            commands::check_permissions,
        ])
        .setup(|app| {
            // Set up the system tray
            tray::setup::create_tray(app)?;

            // Initialize database
            let app_handle = app.handle().clone();
            let state = app_handle.state::<Arc<AppState>>();
            state.init_db()?;

            // Create overlay window (hidden, transparent, click-through)
            let overlay = WebviewWindowBuilder::new(
                app,
                "overlay",
                WebviewUrl::App("index.html".into()),
            )
            .title("")
            .inner_size(300.0, 60.0)
            .resizable(false)
            .decorations(false)
            .transparent(true)
            .always_on_top(true)
            .skip_taskbar(true)
            .visible(false)
            .focused(false)
            .build()?;

            overlay.set_ignore_cursor_events(true)?;

            // Position top-center below menu bar
            if let Ok(Some(monitor)) = overlay.primary_monitor() {
                let size = monitor.size();
                let x = (size.width as f64 / 2.0 - 150.0) as i32;
                let _ = overlay.set_position(tauri::Position::Physical(
                    tauri::PhysicalPosition::new(x, 40),
                ));
            }

            // Register global hotkey for recording
            let shortcut_str = state.config.read().shortcut.clone();
            if let Err(e) = app.handle().global_shortcut().on_shortcut(
                shortcut_str.as_str(),
                |app_handle, _shortcut, event| {
                    let state = app_handle.state::<Arc<AppState>>();
                    let mode = state.config.read().hotkey_mode.clone();
                    let is_recording = state.audio_thread.lock().is_some();

                    let should_start = matches!(
                        (&mode, &event.state, is_recording),
                        (HotkeyMode::PushToTalk, ShortcutState::Pressed, false)
                            | (HotkeyMode::Toggle, ShortcutState::Pressed, false)
                    ) && state.get_state() == "idle";

                    let should_stop = matches!(
                        (&mode, &event.state, is_recording),
                        (HotkeyMode::PushToTalk, ShortcutState::Released, true)
                            | (HotkeyMode::Toggle, ShortcutState::Pressed, true)
                    );

                    if should_start {
                        log::info!("Hotkey: starting recording");
                        if let Err(e) = commands::do_start_recording(state.inner()) {
                            log::error!("Hotkey start recording failed: {}", e);
                        } else {
                            let _ = app_handle.emit("recording-state-changed", "recording");
                            if state.config.read().show_overlay {
                                windows::show_overlay(app_handle);
                            }
                        }
                    }

                    if should_stop {
                        log::info!("Hotkey: stopping recording, will transcribe and inject");
                        let app = app_handle.clone();
                        let state = Arc::clone(state.inner());
                        std::thread::spawn(move || {
                            let _ = app.emit("recording-state-changed", "transcribing");
                            match commands::do_stop_and_transcribe(&state) {
                                Ok(text)
                                    if !text.is_empty() && text != "[BLANK_AUDIO]" =>
                                {
                                    let _ = app.emit("transcription-complete", &text);

                                    // Save to history DB
                                    save_to_history(&state, &text);

                                    // Inject text into focused app
                                    let method =
                                        state.config.read().injection_method.clone();
                                    let result = match method {
                                        InjectionMethod::Clipboard => {
                                            inject::clipboard::inject_via_clipboard(&text)
                                        }
                                        InjectionMethod::Keyboard => {
                                            inject::keyboard::inject_via_keyboard(&text)
                                        }
                                    };
                                    if let Err(e) = result {
                                        log::error!("Text injection failed: {}", e);
                                    }
                                }
                                Err(e) => log::error!("Transcription failed: {}", e),
                                _ => {}
                            }
                            let _ = app.emit("recording-state-changed", "idle");
                            windows::hide_overlay(&app);
                        });
                    }
                },
            ) {
                log::error!("Failed to register hotkey '{}': {}", shortcut_str, e);
            } else {
                log::info!("Hotkey registered: {}", shortcut_str);
            }

            log::info!("voice2txt initialized successfully");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running voice2txt");
}

/// Save a transcription to the history database.
fn save_to_history(state: &AppState, text: &str) {
    let entry = db::history::HistoryEntry {
        id: uuid::Uuid::new_v4().to_string(),
        text: text.to_string(),
        duration_ms: 0,
        model: format!("{:?}", state.config.read().model_size),
        created_at: chrono::Utc::now().to_rfc3339(),
        app_name: None,
        word_count: text.split_whitespace().count() as i32,
    };
    let db_lock = state.db.lock();
    if let Some(conn) = db_lock.as_ref() {
        if let Err(e) = db::history::insert(conn, &entry) {
            log::error!("Failed to save history: {}", e);
        }
    }
}
