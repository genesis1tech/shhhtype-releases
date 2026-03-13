mod audio;
mod commands;
mod config;
mod db;
mod hotkey;
mod inject;
mod license;
mod rewrite;
mod sound;
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
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_notification::init())
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
            commands::export_history,
            commands::get_dictionary,
            commands::update_dictionary,
            commands::check_permissions,
            commands::request_microphone_permission,
            commands::get_model_status,
            commands::download_model,
            commands::delete_model,
            commands::rewrite_last_transcription,
            commands::get_groq_usage,
            commands::activate_license,
            commands::get_license_status,
            commands::deactivate_license,
        ])
        .setup(|app| {
            // Set up the system tray
            tray::setup::create_tray(app)?;

            // Initialize database
            let app_handle = app.handle().clone();
            let state = app_handle.state::<Arc<AppState>>();
            state.init_db()?;

            // Request permissions on startup so the app appears in System Settings
            commands::request_microphone_permission();
            commands::request_accessibility_permission();

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
            register_hotkey(app, &shortcut_str);

            // Register rewrite hotkey
            let rewrite_shortcut = state.config.read().rewrite_hotkey.clone();
            register_rewrite_hotkey(app, &rewrite_shortcut);

            // Show welcome/onboarding on first launch
            let onboarding_flag = state.data_dir.join(".onboarding_complete");
            if !onboarding_flag.exists() {
                windows::show_welcome(app.handle());
                // Mark onboarding as shown (user can still go through it)
                let _ = std::fs::write(&onboarding_flag, "1");
            }

            log::info!("vox2txt initialized successfully");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running vox2txt");
}

/// Register (or re-register) the global hotkey.
fn register_hotkey(app: &tauri::App, shortcut_str: &str) {
    if let Err(e) = app.handle().global_shortcut().on_shortcut(
        shortcut_str,
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
                    let config = state.config.read();
                    if config.sound_feedback {
                        sound::play_start_sound();
                    }
                    let _ = app_handle.emit("recording-state-changed", "recording");
                    if config.show_overlay {
                        windows::show_overlay(app_handle);
                    }
                }
            }

            if should_stop {
                log::info!("Hotkey: stopping recording, will transcribe and inject");
                let app = app_handle.clone();
                let state = Arc::clone(state.inner());
                std::thread::spawn(move || {
                    let duration_ms = commands::get_recording_duration_ms(&state);
                    let _ = app.emit("recording-state-changed", "transcribing");
                    match commands::do_stop_and_transcribe(&state) {
                        Ok(text)
                            if !text.is_empty() && text != "[BLANK_AUDIO]" =>
                        {
                            let config = state.config.read().clone();
                            if config.sound_feedback {
                                sound::play_stop_sound();
                            }

                            let _ = app.emit("transcription-complete", &text);

                            // Save to history DB with actual duration
                            save_to_history(&state, &text, duration_ms);

                            // Send macOS notification
                            send_notification(&app, &text);

                            // Inject text into focused app
                            let result = match config.injection_method {
                                InjectionMethod::Clipboard => {
                                    inject::clipboard::inject_via_clipboard(&text)
                                }
                                InjectionMethod::Keyboard => {
                                    let res = inject::keyboard::inject_via_keyboard(&text);
                                    // Auto-copy to clipboard when using keyboard injection
                                    if config.auto_copy {
                                        if let Err(e) = inject::clipboard::copy_to_clipboard(&text) {
                                            log::error!("Auto-copy failed: {}", e);
                                        }
                                    }
                                    res
                                }
                            };
                            if let Err(e) = result {
                                log::error!("Text injection failed: {}", e);
                            }

                            // If rewrite is enabled, keep overlay visible with
                            // click-through disabled so user can click "Rewrite?"
                            if config.rewrite_enabled && config.groq_api_key.is_some() {
                                windows::enable_overlay_interaction(&app);
                                let _ = app.emit("recording-state-changed", "idle");
                                let app_for_hide = app.clone();
                                std::thread::spawn(move || {
                                    std::thread::sleep(std::time::Duration::from_secs(3));
                                    windows::disable_overlay_interaction(&app_for_hide);
                                    windows::hide_overlay(&app_for_hide);
                                });
                                // Skip the normal hide below
                                return;
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
}

/// Register the AI rewrite hotkey.
fn register_rewrite_hotkey(app: &tauri::App, shortcut_str: &str) {
    if let Err(e) = app.handle().global_shortcut().on_shortcut(
        shortcut_str,
        |app_handle, _shortcut, event| {
            if event.state != ShortcutState::Pressed {
                return;
            }

            let state = app_handle.state::<Arc<AppState>>();

            // Only rewrite when idle and there's a last transcription
            if state.get_state() != "idle" {
                return;
            }
            if state.last_transcription.lock().is_none() {
                return;
            }

            let config = state.config.read().clone();
            if !config.rewrite_enabled {
                log::info!("Rewrite hotkey pressed but rewrite is disabled");
                return;
            }
            if config.groq_api_key.as_deref().unwrap_or("").is_empty() {
                log::warn!("Rewrite hotkey pressed but no Groq API key set");
                return;
            }

            log::info!("Rewrite hotkey pressed, rewriting last transcription");
            let app = app_handle.clone();
            let state = Arc::clone(state.inner());
            std::thread::spawn(move || {
                let _ = app.emit("recording-state-changed", "transcribing");
                windows::show_overlay(&app);
                let _ = app.emit("rewrite-started", ());

                let text = state.last_transcription.lock().clone().unwrap_or_default();
                let config = state.config.read().clone();

                match rewrite::rewrite_text(&text, &config.rewrite_style, config.groq_api_key.as_deref().unwrap_or(""), Some(&state.groq_usage)) {
                    Ok(rewritten) => {
                        log::info!("Rewrite complete: {}", rewritten);
                        // Undo original paste, inject rewritten text
                        if let Err(e) = commands::undo_and_inject(&rewritten, &config.injection_method) {
                            log::error!("Rewrite injection failed: {}", e);
                        }
                        *state.last_transcription.lock() = Some(rewritten.clone());
                        let _ = app.emit("rewrite-complete", &rewritten);
                    }
                    Err(e) => {
                        log::error!("Rewrite failed: {}", e);
                        let _ = app.emit("rewrite-error", e.to_string());
                    }
                }

                let _ = app.emit("recording-state-changed", "idle");
                // Brief delay before hiding overlay so user sees result
                std::thread::sleep(std::time::Duration::from_millis(1500));
                windows::hide_overlay(&app);
            });
        },
    ) {
        log::error!("Failed to register rewrite hotkey '{}': {}", shortcut_str, e);
    } else {
        log::info!("Rewrite hotkey registered: {}", shortcut_str);
    }
}

/// Save a transcription to the history database.
fn save_to_history(state: &AppState, text: &str, duration_ms: i64) {
    let entry = db::history::HistoryEntry {
        id: uuid::Uuid::new_v4().to_string(),
        text: text.to_string(),
        duration_ms,
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

/// Send a macOS notification with transcription preview.
fn send_notification(app: &tauri::AppHandle, text: &str) {
    use tauri_plugin_notification::NotificationExt;
    let preview = if text.len() > 80 {
        format!("{}...", &text[..77])
    } else {
        text.to_string()
    };
    let _ = app
        .notification()
        .builder()
        .title("vox2txt")
        .body(preview)
        .show();
}
