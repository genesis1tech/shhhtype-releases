mod analytics;
mod audio;
mod commands;
mod config;
mod db;
mod hotkey;
mod inject;
mod license;
mod rewrite;
mod skills;
mod sound;
mod state;
mod transcribe;
mod tray;
mod update;
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
            commands::rewrite_and_inject,
            commands::get_groq_usage,
            commands::activate_license,
            commands::get_license_status,
            commands::deactivate_license,
            commands::get_trial_info,
            commands::clear_composition,
            commands::get_composition_count,
            commands::list_audio_devices,
            commands::list_skills,
            commands::restart_app,
            commands::open_url,
            commands::check_for_updates,
            commands::get_update_info,
        ])
        .setup(|app| {
            // Set up the system tray
            tray::setup::create_tray(app)?;

            // Initialize database
            let app_handle = app.handle().clone();
            let state = app_handle.state::<Arc<AppState>>();
            state.init_db()?;

            // Ensure trial start date is recorded in keychain
            license::ensure_trial_start(&state.data_dir);

            // Validate license with LemonSqueezy (every 24h, background)
            {
                let data_dir = state.data_dir.clone();
                std::thread::spawn(move || {
                    license::validate_license_online(&data_dir);
                });
            }

            // Ensure default skills exist and load them
            skills::ensure_default_skills(&state.data_dir);
            *state.skills.lock() = skills::load_skills(&state.data_dir);

            // Track app launch
            analytics::track("app_launched", serde_json::json!({}));

            // Request accessibility permission on startup so the app appears in
            // System Settings > Accessibility. Microphone permission is requested
            // via the onboarding wizard or when the user first tries to record.
            commands::request_accessibility_permission();

            // Reconcile auto-launch setting with actual OS autostart state
            {
                use tauri_plugin_autostart::ManagerExt;
                let manager = app.handle().autolaunch();
                let config_auto_launch = state.config.read().auto_launch;
                match manager.is_enabled() {
                    Ok(os_enabled) if os_enabled != config_auto_launch => {
                        let result = if config_auto_launch {
                            manager.enable()
                        } else {
                            manager.disable()
                        };
                        if let Err(e) = result {
                            log::warn!("Failed to reconcile auto-launch state: {}", e);
                        } else {
                            log::info!("Auto-launch reconciled to {}", config_auto_launch);
                        }
                    }
                    Err(e) => log::warn!("Failed to check auto-launch state: {}", e),
                    _ => {} // already in sync
                }
            }

            // Create overlay window (hidden, transparent, click-through)
            let overlay = WebviewWindowBuilder::new(
                app,
                "overlay",
                WebviewUrl::App("index.html".into()),
            )
            .title("")
            .inner_size(360.0, 60.0)
            .resizable(false)
            .decorations(false)
            .transparent(true)
            // NOTE: do NOT use .always_on_top(true) — it sets a low Tauri-managed
            // window level that conflicts with our manual setLevel_ for full-screen overlay.
            .skip_taskbar(true)
            .visible(false)
            .focused(false)
            .build()?;

            overlay.set_ignore_cursor_events(true)?;

            // Swizzle the overlay NSWindow to NSPanel so it can appear over
            // full-screen apps. Only NSPanel is allowed in full-screen Spaces.
            #[cfg(target_os = "macos")]
            {
                use cocoa::base::id;
                let ns_window: id = overlay.ns_window().unwrap() as id;
                unsafe {
                    windows::swizzle_to_nspanel(ns_window);
                }
            }

            // Position top-center below menu bar
            if let Ok(Some(monitor)) = overlay.primary_monitor() {
                let size = monitor.size();
                let x = (size.width as f64 / 2.0 - 180.0) as i32;
                let _ = overlay.set_position(tauri::Position::Physical(
                    tauri::PhysicalPosition::new(x, 40),
                ));
            }

            // Warm up the overlay webview by briefly showing and hiding it.
            // On macOS, the first show of a webview window can activate the app
            // and steal focus from the user's target app. By doing this warmup
            // during startup, the overlay is fully initialized before the user
            // ever records, preventing focus theft on first use.
            {
                let app_for_warmup = app.handle().clone();
                std::thread::spawn(move || {
                    // Short delay to let the webview finish loading
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    windows::show_overlay(&app_for_warmup, false);
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    windows::hide_overlay(&app_for_warmup);
                    log::info!("Overlay warmup complete");
                });
            }

            // Register global hotkey for recording
            let shortcut_str = state.config.read().shortcut.clone();
            register_hotkey(&app.handle().clone(), &shortcut_str);

            // Register rewrite hotkey
            let rewrite_shortcut = state.config.read().rewrite_hotkey.clone();
            register_rewrite_hotkey(&app.handle().clone(), &rewrite_shortcut);

            // Show welcome/onboarding on first launch
            let onboarding_flag = state.data_dir.join(".onboarding_complete");
            if !onboarding_flag.exists() {
                windows::show_welcome(app.handle());
                // Mark onboarding as shown (user can still go through it)
                let _ = std::fs::write(&onboarding_flag, "1");
            }

            // Background timer: auto-clear composition buffer after TTL (10 min)
            {
                let app_for_ttl = app.handle().clone();
                std::thread::spawn(move || {
                    loop {
                        std::thread::sleep(std::time::Duration::from_secs(60));
                        let state = app_for_ttl.state::<Arc<AppState>>();
                        let expired = state.composition.lock().is_expired();
                        if expired {
                            state.composition.lock().clear();
                            *state.last_transcription.lock() = None;
                            tray::setup::update_tray_segment_count(&app_for_ttl, 0);
                            log::info!("Composition buffer auto-cleared (TTL expired)");
                        }
                    }
                });
            }

            // Check for updates in the background after a short delay
            {
                let app_for_update = app.handle().clone();
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_secs(5));
                    let state = app_for_update.state::<Arc<AppState>>();
                    match update::check_for_update() {
                        Ok(Some(release)) => {
                            log::info!("Update available: {}", release.tag_name);
                            *state.latest_release.lock() = Some(release.clone());
                            let _ = app_for_update.emit("update-available", &release);
                            tray::setup::show_update_available(&app_for_update, &release);
                        }
                        Ok(None) => log::info!("App is up to date"),
                        Err(e) => log::warn!("Update check failed: {}", e),
                    }
                });
            }

            log::info!("ShhhType initialized successfully");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running ShhhType");
}

/// Register (or re-register) the global recording hotkey.
pub fn register_hotkey(app: &tauri::AppHandle, shortcut_str: &str) {
    if let Err(e) = app.global_shortcut().on_shortcut(
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
                // Block recording when trial has expired
                if !license::is_app_usable(&state.data_dir) {
                    log::warn!("Hotkey: trial expired, recording blocked");
                    let _ = app_handle.emit("recording-error", "Your 7-day trial has expired. Subscribe to continue using ShhhType.");
                    return;
                }
                log::info!("Hotkey: starting recording");
                if let Err(e) = commands::do_start_recording(state.inner(), Some(app_handle.clone())) {
                    log::error!("Hotkey start recording failed: {}", e);
                    let _ = app_handle.emit("recording-error", e);
                } else {
                    let config = state.config.read();
                    if config.sound_feedback {
                        sound::play_start_sound();
                    }
                    let _ = app_handle.emit("recording-state-changed", "recording");
                    if config.show_overlay {
                        // Bump generation so any pending hide timer from a previous cycle is invalidated
                        state.overlay_generation.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        let inline = config.overlay_position == crate::config::settings::OverlayPosition::Inline;
                        windows::show_overlay(app_handle, inline);
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

                            // Save to history DB with actual duration
                            save_to_history(&state, &text, duration_ms);

                            // Track transcription analytics
                            analytics::track("transcription_completed", serde_json::json!({
                                "backend": format!("{:?}", config.transcription_backend),
                                "duration_ms": duration_ms,
                                "word_count": text.split_whitespace().count(),
                                "char_count": text.chars().count(),
                            }));

                            // Send macOS notification
                            send_notification(&app, &text);

                            // Inject text into focused app
                            let injection_ok = match config.injection_method {
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

                            // Only stage composition after injection succeeds
                            if injection_ok.is_ok() {
                                *state.last_transcription.lock() = Some(text.clone());
                                state.composition.lock().append(text.clone(), config.injection_method.clone());
                            } else {
                                let err = injection_ok.unwrap_err();
                                log::error!("Text injection failed: {}", err);
                                // Notify frontend that text was only copied, not injected
                                let _ = app.emit("injection-copy-only", err.to_string());
                            }

                            let segment_count = state.composition.lock().len();
                            if config.rewrite_enabled {
                                tray::setup::update_tray_segment_count(&app, segment_count);
                            }
                            let _ = app.emit(
                                "transcription-complete",
                                commands::TranscriptionCompletePayload {
                                    text: text.clone(),
                                    segment_count,
                                },
                            );

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
pub fn register_rewrite_hotkey(app: &tauri::AppHandle, shortcut_str: &str) {
    if let Err(e) = app.global_shortcut().on_shortcut(
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
            // AI rewrite requires a valid license or active trial
            if !license::is_app_usable(&state.data_dir) {
                log::warn!("Rewrite hotkey pressed but trial expired / no valid license");
                return;
            }
            if config.groq_api_key.as_deref().unwrap_or("").is_empty() {
                log::warn!("Rewrite hotkey pressed but no Groq API key set");
                return;
            }

            log::info!("Rewrite hotkey pressed, rewriting last transcription");
            let app = app_handle.clone();
            let state = Arc::clone(state.inner());
            let inline = config.overlay_position == crate::config::settings::OverlayPosition::Inline;
            // Bump generation to invalidate any pending 3s hide timer from recording
            state.overlay_generation.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            std::thread::spawn(move || {
                let _ = app.emit("recording-state-changed", "transcribing");
                windows::show_overlay(&app, inline);
                let _ = app.emit("rewrite-started", ());

                // Read from composition buffer; fall back to last_transcription
                let (text, is_multi, segment_count) = {
                    let buf = state.composition.lock();
                    if buf.len() == 0 {
                        let last = state.last_transcription.lock().clone().unwrap_or_default();
                        (last, false, 1usize)
                    } else {
                        (buf.join(), buf.is_multi(), buf.len())
                    }
                };
                let config = state.config.read().clone();

                // Read injected char count for selection-based replacement
                let char_count = {
                    let buf = state.composition.lock();
                    if buf.len() == 0 {
                        text.chars().count()
                    } else {
                        buf.injected_chars()
                    }
                };

                // Check for skill trigger in the text
                let (rewrite_text_input, custom_prompt) = {
                    let skills = state.skills.lock();
                    match skills::detect_skill(&text, &skills) {
                        Some(skill_match) => {
                            log::info!("Skill detected: {}", skill_match.skill.name);
                            analytics::track("skill_used", serde_json::json!({
                                "skill_name": skill_match.skill.name,
                            }));
                            (skill_match.cleaned_text, Some(skill_match.skill.system_prompt))
                        }
                        None => (text.clone(), None),
                    }
                };

                match rewrite::rewrite_text(&rewrite_text_input, &config.rewrite_style, config.groq_api_key.as_deref().unwrap_or(""), Some(&state.groq_usage), custom_prompt.as_deref()) {
                    Ok(rewritten) => {
                        log::info!("Rewrite complete (multi={}, chars={}): {}", is_multi, char_count, rewritten);
                        analytics::track("rewrite_completed", serde_json::json!({
                            "style": format!("{:?}", config.rewrite_style),
                            "has_skill": custom_prompt.is_some(),
                        }));

                        // Select-back and replace the original injected text
                        if let Err(e) = commands::select_back_and_inject(char_count, &rewritten) {
                            log::error!("Rewrite injection failed, falling back to clipboard: {}", e);
                            let _ = crate::inject::clipboard::copy_to_clipboard(&rewritten);
                            let _ = app.emit("rewrite-fallback", "Copied to clipboard");
                        }

                        // Clear composition buffer and last_transcription to prevent re-rewriting
                        state.composition.lock().clear();
                        *state.last_transcription.lock() = None;
                        tray::setup::update_tray_segment_count(&app, 0);
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
    let preview = if text.chars().count() > 80 {
        let truncated: String = text.chars().take(77).collect();
        format!("{}...", truncated)
    } else {
        text.to_string()
    };
    let _ = app
        .notification()
        .builder()
        .title("ShhhType")
        .body(preview)
        .show();
}
