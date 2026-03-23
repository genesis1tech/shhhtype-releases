use crate::audio::capture::AudioCapture;
use crate::audio::resampler::resample_to_16khz;
use cpal::traits::{DeviceTrait, HostTrait};
use crate::config::settings::{RewriteStyle, Settings, TranscriptionBackend};
use crate::db::history::{HistoryEntry, HistoryQuery};
use crate::license::{self, LicenseStatus};
use crate::state::{AppState, GroqUsage, STATE_IDLE, STATE_RECORDING, STATE_TRANSCRIBING};
use crate::transcribe::dictionary::DictionaryEntry;
use crate::transcribe::model::{self, ModelSize, ModelStatus};
use crate::vad::energy::EnergyVad;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Instant;
use tauri::{AppHandle, Emitter, State};

/// Payload emitted with `transcription-complete` event.
#[derive(Clone, serde::Serialize)]
pub struct TranscriptionCompletePayload {
    pub text: String,
    pub segment_count: usize,
}

/// Start audio capture. Shared by both the Tauri command and the hotkey handler.
/// `app` is optional — when provided, waveform levels are emitted to the frontend.
pub fn do_start_recording(state: &Arc<AppState>, app: Option<AppHandle>) -> Result<(), String> {
    if state.audio_thread.lock().is_some() {
        return Err("Already recording".into());
    }

    // Check microphone permission before attempting to record
    if !check_microphone_permission() {
        log::error!("Microphone permission not granted — cannot record");
        return Err("Microphone permission not granted. Enable it in System Settings > Privacy & Security > Microphone.".into());
    }

    state.audio_buffer.lock().clear();
    state.audio_stop_flag.store(false, Ordering::Relaxed);
    state.set_state(STATE_RECORDING);
    *state.recording_started_at.lock() = Some(Instant::now());

    let buffer = Arc::clone(&state.audio_buffer);
    let stop_flag = Arc::clone(&state.audio_stop_flag);
    let sample_rate_out = Arc::clone(&state.audio_sample_rate);
    let config = state.config.read().clone();
    let vad_threshold = config.vad_threshold;
    // Convert seconds to frame count (each frame ~50ms)
    let vad_silence_frames = (config.vad_silence_timeout / 0.05).round() as usize;

    let audio_device = config.audio_input_device.clone();

    let handle = std::thread::spawn(move || {
        let mut capture = AudioCapture::new();
        if let Err(e) = capture.start(buffer.clone(), audio_device.as_deref()) {
            log::error!("Audio capture failed to start: {}", e);
            return;
        }

        let sr = capture.sample_rate();
        sample_rate_out.store(sr, Ordering::Relaxed);
        log::info!("Audio capture thread running at {}Hz, silence timeout: {}s ({} frames)",
            sr, config.vad_silence_timeout, vad_silence_frames);

        let mut vad = EnergyVad::new(vad_threshold, vad_silence_frames);
        let mut has_speech = false;
        const WAVEFORM_BARS: usize = 24;
        // Scale VAD chunk size to actual sample rate (~50ms of audio)
        let vad_chunk_size = (sr as usize / 20).max(800); // sr/20 = 50ms worth of samples

        while !stop_flag.load(Ordering::Relaxed) {
            std::thread::sleep(std::time::Duration::from_millis(50));

            // Check VAD on recent audio and compute waveform levels
            let levels: Option<Vec<f32>> = {
                let buf = buffer.lock();
                let len = buf.len();
                if len > vad_chunk_size {
                    // Check last ~50ms of audio (scaled to actual sample rate)
                    let chunk_start = len.saturating_sub(vad_chunk_size);
                    let chunk = &buf[chunk_start..len];
                    let is_speech = vad.is_speech(chunk);
                    if is_speech {
                        has_speech = true;
                    }
                    if has_speech && !is_speech {
                        log::info!("VAD: silence detected, auto-stopping");
                        stop_flag.store(true, Ordering::Relaxed);
                    }

                    // Compute waveform levels while we hold the lock
                    if app.is_some() {
                        let wave_start = len.saturating_sub(vad_chunk_size * 4);
                        let wave_chunk = &buf[wave_start..len];
                        let bar_size = wave_chunk.len() / WAVEFORM_BARS;
                        Some((0..WAVEFORM_BARS)
                            .map(|i| {
                                let start = i * bar_size;
                                let end = (start + bar_size).min(wave_chunk.len());
                                let rms: f32 = wave_chunk[start..end]
                                    .iter()
                                    .map(|s| s * s)
                                    .sum::<f32>()
                                    / (end - start) as f32;
                                (rms.sqrt() * 15.0).min(1.0)
                            })
                            .collect())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }; // lock dropped here

            // Emit waveform levels outside the lock
            if let (Some(ref app), Some(ref levels)) = (&app, &levels) {
                let max_level = levels.iter().cloned().fold(0.0f32, f32::max);
                if max_level > 0.01 {
                    log::debug!("audio-levels max={:.3}", max_level);
                }
                let _ = app.emit("audio-levels", levels);
            }
        }

        capture.stop();
        log::info!("Audio capture thread exiting");
    });

    *state.audio_thread.lock() = Some(handle);
    log::info!("Recording started");
    Ok(())
}

/// Stop recording and transcribe. Shared by both the Tauri command and the hotkey handler.
pub fn do_stop_and_transcribe(state: &Arc<AppState>) -> Result<String, String> {
    state.audio_stop_flag.store(true, Ordering::Relaxed);

    if let Some(handle) = state.audio_thread.lock().take() {
        handle.join().map_err(|_| "Audio thread panicked")?;
    }

    let sample_rate = state.audio_sample_rate.load(Ordering::Relaxed);
    state.set_state(STATE_TRANSCRIBING);
    log::info!("Recording stopped, transcribing...");

    let raw_samples = {
        let mut buf = state.audio_buffer.lock();
        std::mem::take(&mut *buf)
    };

    if raw_samples.is_empty() {
        state.set_state(STATE_IDLE);
        return Err("No audio captured".into());
    }

    log::info!(
        "Captured {} samples at {}Hz ({:.1}s)",
        raw_samples.len(),
        sample_rate,
        raw_samples.len() as f32 / sample_rate as f32
    );

    // Check if the recording contains any speech by finding the peak RMS
    // across 50ms windows. Using overall RMS can miss speech in recordings
    // with long silent sections (the average gets diluted).
    // Whisper hallucinates phrases like "Thank you" when fed pure silence.
    let silence_floor = 0.002;
    let window_size = (sample_rate as usize / 20).max(800); // ~50ms
    let peak_rms = raw_samples
        .chunks(window_size)
        .map(|chunk| crate::vad::energy::EnergyVad::rms(chunk))
        .fold(0.0f32, f32::max);
    if peak_rms < silence_floor {
        log::info!(
            "Peak audio RMS ({:.6}) below silence floor ({:.6}), skipping transcription (silence only)",
            peak_rms,
            silence_floor
        );
        state.set_state(STATE_IDLE);
        return Ok(String::new());
    }
    log::info!("Peak audio RMS: {:.6} (silence floor: {:.6})", peak_rms, silence_floor);

    let config = state.config.read().clone();
    let transcribed_text = match config.transcription_backend {
        TranscriptionBackend::Cloud => {
            // Cloud: send raw audio at native sample rate — Groq downsamples server-side.
            // This skips the expensive client-side resampling step entirely.
            let api_key = config.groq_api_key.as_deref().unwrap_or("");
            if api_key.is_empty() {
                state.set_state(STATE_IDLE);
                return Err("Groq API key not set. Configure it in Settings > General.".into());
            }
            log::info!("Transcribing via Groq cloud (skipping resample, sending {}Hz)...", sample_rate);
            crate::transcribe::groq::transcribe(&raw_samples, sample_rate, &config.language, api_key, Some(&state.groq_usage))
                .map_err(|e| format!("Cloud transcription failed: {}", e))?
        }
        TranscriptionBackend::Local => {
            // Local Whisper requires 16kHz — resample if needed
            let resample_start = Instant::now();
            let samples_16k = resample_to_16khz(&raw_samples, sample_rate)
                .map_err(|e| format!("Resampling failed: {}", e))?;
            log::info!("Resampled to {} samples at 16kHz in {:?}", samples_16k.len(), resample_start.elapsed());

            {
                let mut engine = state.whisper_engine.lock();
                if !engine.is_loaded() {
                    let model_path = model::model_path(&state.data_dir, &config.model_size);
                    if !model_path.exists() {
                        state.set_state(STATE_IDLE);
                        return Err(format!(
                            "Model not found: {}. Please download a model first.",
                            model_path.display()
                        ));
                    }
                    log::info!("Loading whisper model: {}", model_path.display());
                    engine
                        .load_model(&model_path)
                        .map_err(|e| format!("Failed to load model: {}", e))?;
                }
            }

            state
                .whisper_engine
                .lock()
                .transcribe(&samples_16k, &config.language)
                .map_err(|e| format!("Transcription failed: {}", e))?
        }
    };

    let final_text = {
        let mut cache = state.dictionary_cache.lock();
        let dict = cache.get_or_insert_with(|| {
            let dict_path = state.data_dir.join("dictionary.json");
            crate::transcribe::dictionary::Dictionary::load(&dict_path).unwrap_or_default()
        });
        dict.correct(&transcribed_text)
    };

    state.set_state(STATE_IDLE);
    log::info!("Transcription complete: {}", final_text);
    Ok(final_text)
}

/// Get recording duration in ms since start.
pub fn get_recording_duration_ms(state: &AppState) -> i64 {
    state
        .recording_started_at
        .lock()
        .map(|started| started.elapsed().as_millis() as i64)
        .unwrap_or(0)
}

#[tauri::command]
pub fn start_recording(
    app: AppHandle,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    do_start_recording(state.inner(), Some(app.clone()))?;
    let _ = app.emit("recording-state-changed", "recording");
    if state.config.read().show_overlay {
        crate::windows::show_overlay(&app);
    }
    Ok(())
}

#[tauri::command]
pub fn stop_recording(
    app: AppHandle,
    state: State<'_, Arc<AppState>>,
) -> Result<String, String> {
    let _ = app.emit("recording-state-changed", "transcribing");
    let text = do_stop_and_transcribe(state.inner())?;
    let segment_count = state.composition.lock().len();
    let _ = app.emit("recording-state-changed", "idle");
    let _ = app.emit("transcription-complete", TranscriptionCompletePayload {
        text: text.clone(),
        segment_count,
    });
    crate::windows::hide_overlay(&app);
    Ok(text)
}

#[tauri::command]
pub fn cancel_recording(
    app: AppHandle,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    state.audio_stop_flag.store(true, Ordering::Relaxed);
    if let Some(handle) = state.audio_thread.lock().take() {
        let _ = handle.join();
    }
    state.set_state(STATE_IDLE);
    state.audio_buffer.lock().clear();
    *state.recording_started_at.lock() = None;
    let _ = app.emit("recording-state-changed", "idle");
    crate::windows::hide_overlay(&app);
    log::info!("Recording cancelled");
    Ok(())
}

#[tauri::command]
pub fn get_recording_state(state: State<'_, Arc<AppState>>) -> String {
    state.get_state().to_string()
}

#[tauri::command]
pub fn get_settings(state: State<'_, Arc<AppState>>) -> Settings {
    state.config.read().clone()
}

#[tauri::command]
pub fn update_settings(
    app: tauri::AppHandle,
    state: State<'_, Arc<AppState>>,
    settings: Settings,
) -> Result<(), String> {
    let old = state.config.read().clone();
    let hotkey_changed = old.shortcut != settings.shortcut || old.rewrite_hotkey != settings.rewrite_hotkey;

    let rewrite_was_enabled = old.rewrite_enabled;
    settings.save(&state.data_dir).map_err(|e| e.to_string())?;
    *state.config.write() = settings.clone();

    // Clear tray segment count when rewrite is toggled off
    if rewrite_was_enabled && !settings.rewrite_enabled {
        crate::tray::setup::update_tray_segment_count(&app, 0);
    }

    // Global hotkeys are registered once at startup via the OS Carbon API.
    // The tauri-plugin-global-shortcut plugin cannot reliably re-register
    // shortcuts at runtime, so notify the frontend that a restart is needed.
    if hotkey_changed {
        log::info!("Hotkey changed — restart required to take effect");
        let _ = app.emit("hotkey-restart-required", ());
    }

    Ok(())
}

#[tauri::command]
pub fn restart_app(app: tauri::AppHandle) {
    app.restart();
}

#[tauri::command]
pub fn get_history(
    state: State<'_, Arc<AppState>>,
    query: HistoryQuery,
) -> Result<Vec<HistoryEntry>, String> {
    let db_lock = state.db.lock();
    let conn = db_lock.as_ref().ok_or("Database not initialized")?;
    crate::db::history::query(conn, &query).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_history_entry(state: State<'_, Arc<AppState>>, id: String) -> Result<(), String> {
    let db_lock = state.db.lock();
    let conn = db_lock.as_ref().ok_or("Database not initialized")?;
    crate::db::history::delete(conn, &id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn export_history(state: State<'_, Arc<AppState>>) -> Result<Vec<HistoryEntry>, String> {
    let db_lock = state.db.lock();
    let conn = db_lock.as_ref().ok_or("Database not initialized")?;
    crate::db::history::query(
        conn,
        &HistoryQuery {
            search: None,
            limit: Some(10000),
            offset: Some(0),
        },
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_dictionary(state: State<'_, Arc<AppState>>) -> Vec<DictionaryEntry> {
    let dict_path = state.data_dir.join("dictionary.json");
    crate::transcribe::dictionary::Dictionary::load(&dict_path)
        .unwrap_or_default()
        .entries()
}

#[tauri::command]
pub fn update_dictionary(
    state: State<'_, Arc<AppState>>,
    entries: Vec<DictionaryEntry>,
) -> Result<(), String> {
    let dict_path = state.data_dir.join("dictionary.json");
    let dict = crate::transcribe::dictionary::Dictionary::from_entries(entries);
    dict.save(&dict_path).map_err(|e| e.to_string())?;
    // Invalidate cache so next transcription picks up changes
    *state.dictionary_cache.lock() = None;
    Ok(())
}

#[tauri::command]
pub fn check_permissions() -> PermissionStatus {
    PermissionStatus {
        microphone: check_microphone_permission(),
        accessibility: check_accessibility_permission(),
    }
}

#[derive(serde::Serialize)]
pub struct PermissionStatus {
    pub microphone: bool,
    pub accessibility: bool,
}

fn check_microphone_permission() -> bool {
    #[cfg(target_os = "macos")]
    {
        use objc::runtime::{Class, Object};
        use objc::{msg_send, sel, sel_impl};
        unsafe {
            let cls = match Class::get("AVCaptureDevice") {
                Some(c) => c,
                None => return false,
            };
            let ns_string_cls = match Class::get("NSString") {
                Some(c) => c,
                None => return false,
            };
            let audio_str: *const Object =
                msg_send![ns_string_cls, stringWithUTF8String: b"soun\0".as_ptr()];
            let status: i64 = msg_send![cls, authorizationStatusForMediaType: audio_str];
            // 0 = notDetermined, 1 = restricted, 2 = denied, 3 = authorized
            status == 3
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        false
    }
}

#[tauri::command]
pub fn request_microphone_permission() {
    #[cfg(target_os = "macos")]
    {
        // Spawn a short-lived audio stream via cpal to trigger the macOS microphone permission prompt.
        // This registers the app in System Settings > Privacy > Microphone.
        std::thread::spawn(|| {
            use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
            let host = cpal::default_host();
            if let Some(device) = host.default_input_device() {
                if let Ok(config) = device.default_input_config() {
                    if let Ok(stream) = device.build_input_stream_raw(
                        &config.into(),
                        cpal::SampleFormat::F32,
                        move |_data: &cpal::Data, _info: &cpal::InputCallbackInfo| {},
                        move |err| { log::error!("Permission stream error: {}", err); },
                        None,
                    ) {
                        let _ = stream.play();
                        std::thread::sleep(std::time::Duration::from_millis(200));
                        drop(stream);
                    }
                }
            }
            log::info!("Microphone permission request triggered");
        });
    }
}

/// Request accessibility permission by showing the macOS system prompt.
/// This uses AXIsProcessTrustedWithOptions with kAXTrustedCheckOptionPrompt=true,
/// which shows a dialog directing the user to System Settings > Accessibility.
pub fn request_accessibility_permission() {
    #[cfg(target_os = "macos")]
    {
        use core_foundation::base::TCFType;
        use core_foundation::boolean::CFBoolean;
        use core_foundation::dictionary::CFDictionary;
        use core_foundation::string::CFString;

        extern "C" {
            fn AXIsProcessTrustedWithOptions(options: core_foundation::base::CFTypeRef) -> bool;
        }

        let key = CFString::new("AXTrustedCheckOptionPrompt");
        let value = CFBoolean::true_value();
        let options = CFDictionary::from_CFType_pairs(&[(key.as_CFType(), value.as_CFType())]);

        let trusted = unsafe { AXIsProcessTrustedWithOptions(options.as_CFTypeRef()) };
        log::info!("Accessibility permission request triggered (currently trusted: {})", trusted);
    }
}

fn check_accessibility_permission() -> bool {
    #[cfg(target_os = "macos")]
    {
        extern "C" {
            fn AXIsProcessTrusted() -> bool;
        }
        unsafe { AXIsProcessTrusted() }
    }
    #[cfg(not(target_os = "macos"))]
    {
        false
    }
}

#[tauri::command]
pub fn get_model_status(state: State<'_, Arc<AppState>>) -> Vec<ModelStatus> {
    model::get_all_model_status(&state.data_dir)
}

#[tauri::command]
pub async fn download_model(
    app: AppHandle,
    state: State<'_, Arc<AppState>>,
    model_size: ModelSize,
) -> Result<(), String> {
    let data_dir = state.data_dir.clone();
    model::download_model(app, data_dir, model_size)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_model(
    state: State<'_, Arc<AppState>>,
    model_size: ModelSize,
) -> Result<(), String> {
    let path = model::model_path(&state.data_dir, &model_size);
    if path.exists() {
        std::fs::remove_file(&path).map_err(|e| e.to_string())?;
        log::info!("Deleted model: {}", model_size.id());
    }
    Ok(())
}

#[derive(serde::Serialize, Clone)]
pub struct RewriteResult {
    pub text: String,
    pub is_multi: bool,
}

#[tauri::command]
pub fn rewrite_last_transcription(
    state: State<'_, Arc<AppState>>,
    style: Option<RewriteStyle>,
) -> Result<RewriteResult, String> {
    let config = state.config.read().clone();

    let api_key = config.groq_api_key.as_deref().unwrap_or("");
    if api_key.is_empty() {
        return Err("Groq API key not set. Configure it in Settings > General.".into());
    }

    let (text, is_multi) = {
        let buf = state.composition.lock();
        if buf.len() == 0 {
            let last_text = state.last_transcription.lock().clone();
            (last_text.ok_or("No recent transcription to rewrite")?, false)
        } else {
            (buf.join(), buf.is_multi())
        }
    };

    let rewrite_style = style.unwrap_or(config.rewrite_style);

    // Check for skill trigger
    let (rewrite_input, custom_prompt) = {
        let skills = state.skills.lock();
        match crate::skills::detect_skill(&text, &skills) {
            Some(skill_match) => {
                log::info!("Skill detected: {}", skill_match.skill.name);
                (skill_match.cleaned_text, Some(skill_match.skill.system_prompt))
            }
            None => (text, None),
        }
    };

    log::info!("Rewriting {} segment(s) with style: {:?}", if is_multi { "multiple" } else { "single" }, rewrite_style);

    let rewritten = crate::rewrite::rewrite_text(&rewrite_input, &rewrite_style, api_key, Some(&state.groq_usage), custom_prompt.as_deref())
        .map_err(|e| format!("Rewrite failed: {}", e))?;

    // Clear both buffers after successful rewrite
    state.composition.lock().clear();
    *state.last_transcription.lock() = None;

    log::info!("Rewrite complete: {}", rewritten);
    Ok(RewriteResult { text: rewritten, is_multi })
}

/// Select `char_count` characters backward from cursor, then paste replacement.
/// Uses Shift+Left Arrow to build a selection, then clipboard paste to replace it.
pub fn select_back_and_inject(char_count: usize, text: &str) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        use core_graphics::event::{CGEvent, CGEventFlags};
        use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

        let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState)
            .map_err(|_| "Failed to create event source")?;

        log::info!("Selecting {} chars backward for replacement...", char_count);

        // Left Arrow keycode = 123. Send with Shift flag to extend selection.
        for _i in 0..char_count {
            let key_down = CGEvent::new_keyboard_event(source.clone(), 123, true)
                .map_err(|_| "Failed to create key event")?;
            key_down.set_flags(CGEventFlags::CGEventFlagShift);
            key_down.post(core_graphics::event::CGEventTapLocation::AnnotatedSession);

            let key_up = CGEvent::new_keyboard_event(source.clone(), 123, false)
                .map_err(|_| "Failed to create key event")?;
            key_up.set_flags(CGEventFlags::CGEventFlagShift);
            key_up.post(core_graphics::event::CGEventTapLocation::AnnotatedSession);

            // 2ms delay per arrow key to let the target app process reliably
            // (matches keyboard injection timing in inject/keyboard.rs)
            std::thread::sleep(std::time::Duration::from_millis(2));
        }

        // Wait for selection to settle
        std::thread::sleep(std::time::Duration::from_millis(50));
        log::info!("Selected {} chars, injecting replacement", char_count);
    }

    // Paste rewritten text — replaces the selection
    crate::inject::clipboard::inject_via_clipboard(text).map_err(|e| e.to_string())
}

/// Rewrite and inject: rewrites composition buffer text, then uses selection-based
/// replacement to swap the original injected text with the rewritten version.
/// Called from the overlay "Rewrite?" button. On failure, copies to clipboard as fallback.
///
/// NOTE: Selection-based replacement assumes the cursor hasn't moved since injection.
/// If the user clicks elsewhere or uses arrow keys between injection and rewrite,
/// the selection will be wrong. This is an accepted trade-off — still far more
/// reliable than undo-based replacement.
#[tauri::command]
pub fn rewrite_and_inject(
    app: AppHandle,
    state: State<'_, Arc<AppState>>,
    style: Option<RewriteStyle>,
) -> Result<RewriteResult, String> {
    // Bump generation to invalidate any pending 3s hide timer from recording
    state.overlay_generation.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

    let config = state.config.read().clone();

    let api_key = config.groq_api_key.as_deref().unwrap_or("");
    if api_key.is_empty() {
        return Err("Groq API key not set. Configure it in Settings > General.".into());
    }

    let (text, is_multi, char_count) = {
        let buf = state.composition.lock();
        if buf.len() == 0 {
            let last_text = state.last_transcription.lock().clone();
            let t = last_text.ok_or("No recent transcription to rewrite")?;
            let cc = t.chars().count();
            (t, false, cc)
        } else {
            (buf.join(), buf.is_multi(), buf.injected_chars())
        }
    };

    let rewrite_style = style.unwrap_or(config.rewrite_style);

    // Check for skill trigger
    let (rewrite_input, custom_prompt) = {
        let skills = state.skills.lock();
        match crate::skills::detect_skill(&text, &skills) {
            Some(skill_match) => {
                log::info!("Skill detected: {}", skill_match.skill.name);
                (skill_match.cleaned_text, Some(skill_match.skill.system_prompt))
            }
            None => (text, None),
        }
    };

    log::info!("Rewrite-and-inject: {} segment(s), {} chars, style: {:?}",
        if is_multi { "multiple" } else { "single" }, char_count, rewrite_style);

    let rewritten = crate::rewrite::rewrite_text(&rewrite_input, &rewrite_style, api_key, Some(&state.groq_usage), custom_prompt.as_deref())
        .map_err(|e| format!("Rewrite failed: {}", e))?;

    // Select-back and replace the original injected text
    if let Err(e) = select_back_and_inject(char_count, &rewritten) {
        log::error!("Rewrite injection failed, falling back to clipboard: {}", e);
        // Fallback: copy rewritten text to clipboard and notify frontend
        let _ = crate::inject::clipboard::copy_to_clipboard(&rewritten);
        let _ = app.emit("rewrite-fallback", "Copied to clipboard");
        // Still clear buffers — the rewrite itself succeeded
    }

    // Clear both buffers after successful rewrite
    state.composition.lock().clear();
    *state.last_transcription.lock() = None;
    crate::tray::setup::update_tray_segment_count(&app, 0);

    // Hide overlay after a short delay so user sees "Rewritten" status
    let app_for_hide = app.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(1500));
        crate::windows::disable_overlay_interaction(&app_for_hide);
        crate::windows::hide_overlay(&app_for_hide);
    });

    log::info!("Rewrite-and-inject complete: {}", rewritten);
    Ok(RewriteResult { text: rewritten, is_multi })
}

#[tauri::command]
pub fn get_groq_usage(state: State<'_, Arc<AppState>>) -> GroqUsage {
    state.groq_usage.lock().clone()
}

#[tauri::command]
pub fn activate_license(
    state: State<'_, Arc<AppState>>,
    key: String,
) -> Result<LicenseStatus, String> {
    license::activate_license(&key, &state.data_dir)
        .map(|_| LicenseStatus::Licensed)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_license_status(state: State<'_, Arc<AppState>>) -> LicenseStatus {
    license::check_license(&state.data_dir)
}

#[tauri::command]
pub fn deactivate_license(state: State<'_, Arc<AppState>>) -> Result<(), String> {
    license::deactivate_license(&state.data_dir).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn clear_composition(app: AppHandle, state: State<'_, Arc<AppState>>) {
    state.composition.lock().clear();
    crate::tray::setup::update_tray_segment_count(&app, 0);
    log::info!("Composition buffer cleared");
}

#[tauri::command]
pub fn get_composition_count(state: State<'_, Arc<AppState>>) -> usize {
    state.composition.lock().len()
}

#[tauri::command]
pub fn open_url(url: String) {
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(&url).spawn();
    }
}

#[tauri::command]
pub fn check_for_updates(
    app: AppHandle,
    state: State<'_, Arc<AppState>>,
) -> Option<crate::update::LatestRelease> {
    match crate::update::check_for_update() {
        Ok(Some(release)) => {
            *state.latest_release.lock() = Some(release.clone());
            let _ = app.emit("update-available", &release);
            crate::tray::setup::show_update_available(&app, &release);
            Some(release)
        }
        Ok(None) => {
            *state.latest_release.lock() = None;
            None
        }
        Err(e) => {
            log::error!("Update check failed: {}", e);
            None
        }
    }
}

#[tauri::command]
pub fn get_update_info(
    state: State<'_, Arc<AppState>>,
) -> Option<crate::update::LatestRelease> {
    state.latest_release.lock().clone()
}

/// Audio input device info returned to the frontend.
#[derive(serde::Serialize)]
pub struct AudioDevice {
    pub name: String,
    pub is_default: bool,
}

#[tauri::command]
pub fn list_audio_devices() -> Vec<AudioDevice> {
    let host = cpal::default_host();
    let default_name = host
        .default_input_device()
        .and_then(|d| d.name().ok());
    host.input_devices()
        .map(|devices| {
            devices
                .filter_map(|d| {
                    let name = d.name().ok()?;
                    let is_default = default_name.as_deref() == Some(&name);
                    Some(AudioDevice { name, is_default })
                })
                .collect()
        })
        .unwrap_or_default()
}
