use crate::audio::capture::AudioCapture;
use crate::audio::resampler::resample_to_16khz;
use crate::config::settings::{Settings, TranscriptionBackend};
use crate::db::history::{HistoryEntry, HistoryQuery};
use crate::state::{AppState, STATE_IDLE, STATE_RECORDING, STATE_TRANSCRIBING};
use crate::transcribe::dictionary::DictionaryEntry;
use crate::transcribe::model::{self, ModelSize, ModelStatus};
use crate::vad::energy::EnergyVad;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Instant;
use tauri::{AppHandle, Emitter, State};

/// Start audio capture. Shared by both the Tauri command and the hotkey handler.
pub fn do_start_recording(state: &Arc<AppState>) -> Result<(), String> {
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
    let vad_threshold = state.config.read().vad_threshold;

    let handle = std::thread::spawn(move || {
        let mut capture = AudioCapture::new();
        if let Err(e) = capture.start(buffer.clone()) {
            log::error!("Audio capture failed to start: {}", e);
            return;
        }

        sample_rate_out.store(capture.sample_rate(), Ordering::Relaxed);
        log::info!("Audio capture thread running at {}Hz", capture.sample_rate());

        // VAD: auto-stop after ~1.5s of silence (30 frames * 50ms)
        let mut vad = EnergyVad::new(vad_threshold, 30);
        let mut has_speech = false;

        while !stop_flag.load(Ordering::Relaxed) {
            std::thread::sleep(std::time::Duration::from_millis(50));

            // Check VAD on recent audio
            let buf = buffer.lock();
            let len = buf.len();
            if len > 800 {
                // Check last ~50ms of audio
                let chunk_start = len.saturating_sub(800);
                let chunk = &buf[chunk_start..len];
                let is_speech = vad.is_speech(chunk);
                if is_speech {
                    has_speech = true;
                }
                // Only auto-stop if we've detected speech before (prevents stopping before user starts talking)
                if has_speech && !is_speech {
                    log::info!("VAD: silence detected, auto-stopping");
                    stop_flag.store(true, Ordering::Relaxed);
                }
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
        let samples = buf.clone();
        buf.clear();
        samples
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

    let samples_16k = resample_to_16khz(&raw_samples, sample_rate)
        .map_err(|e| format!("Resampling failed: {}", e))?;

    log::info!("Resampled to {} samples at 16kHz", samples_16k.len());

    let (language, backend, groq_key) = {
        let cfg = state.config.read();
        (
            cfg.language.clone(),
            cfg.transcription_backend.clone(),
            cfg.groq_api_key.clone(),
        )
    };

    let transcribed_text = match backend {
        TranscriptionBackend::Local => {
            {
                let mut engine = state.whisper_engine.lock();
                if !engine.is_loaded() {
                    let config = state.config.read();
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
                .transcribe(&samples_16k, &language)
                .map_err(|e| format!("Transcription failed: {}", e))?
        }
        TranscriptionBackend::Groq => {
            let api_key = groq_key.ok_or_else(|| {
                "Groq API key not configured. Please add it in Settings.".to_string()
            })?;
            log::info!("Transcribing via Groq API (whisper-large-v3-turbo)...");
            crate::transcribe::groq::transcribe(&samples_16k, &language, &api_key)
                .map_err(|e| format!("Groq transcription failed: {}", e))?
        }
    };

    let dict_path = state.data_dir.join("dictionary.json");
    let final_text = if let Ok(dict) = crate::transcribe::dictionary::Dictionary::load(&dict_path)
    {
        dict.correct(&transcribed_text)
    } else {
        transcribed_text
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
    do_start_recording(state.inner())?;
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
    let _ = app.emit("recording-state-changed", "idle");
    let _ = app.emit("transcription-complete", &text);
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
pub fn update_settings(state: State<'_, Arc<AppState>>, settings: Settings) -> Result<(), String> {
    settings.save(&state.data_dir).map_err(|e| e.to_string())?;
    *state.config.write() = settings;
    Ok(())
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
    dict.save(&dict_path).map_err(|e| e.to_string())
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
