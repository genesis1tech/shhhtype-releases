use crate::audio::capture::AudioCapture;
use crate::audio::resampler::resample_to_16khz;
use crate::config::settings::Settings;
use crate::db::history::{HistoryEntry, HistoryQuery};
use crate::state::{AppState, STATE_IDLE, STATE_RECORDING, STATE_TRANSCRIBING};
use crate::transcribe::dictionary::DictionaryEntry;
use crate::transcribe::model;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, State};

/// Start audio capture. Shared by both the Tauri command and the hotkey handler.
pub fn do_start_recording(state: &Arc<AppState>) -> Result<(), String> {
    if state.audio_thread.lock().is_some() {
        return Err("Already recording".into());
    }

    state.audio_buffer.lock().clear();
    state.audio_stop_flag.store(false, Ordering::Relaxed);
    state.set_state(STATE_RECORDING);

    let buffer = Arc::clone(&state.audio_buffer);
    let stop_flag = Arc::clone(&state.audio_stop_flag);
    let sample_rate_out = Arc::clone(&state.audio_sample_rate);

    let handle = std::thread::spawn(move || {
        let mut capture = AudioCapture::new();
        if let Err(e) = capture.start(buffer) {
            log::error!("Audio capture failed to start: {}", e);
            return;
        }

        sample_rate_out.store(capture.sample_rate(), Ordering::Relaxed);
        log::info!("Audio capture thread running at {}Hz", capture.sample_rate());

        while !stop_flag.load(Ordering::Relaxed) {
            std::thread::sleep(std::time::Duration::from_millis(50));
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

    let transcribed_text = state
        .whisper_engine
        .lock()
        .transcribe(&samples_16k)
        .map_err(|e| format!("Transcription failed: {}", e))?;

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
    // TODO: Check actual macOS permissions
    PermissionStatus {
        microphone: false,
        accessibility: false,
    }
}

#[derive(serde::Serialize)]
pub struct PermissionStatus {
    pub microphone: bool,
    pub accessibility: bool,
}
