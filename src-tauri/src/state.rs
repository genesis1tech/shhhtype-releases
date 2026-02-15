use crate::config::settings::Settings;
use crate::transcribe::engine::WhisperEngine;
use parking_lot::{Mutex, RwLock};
use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU8, Ordering};
use std::sync::Arc;

/// Recording states
pub const STATE_IDLE: u8 = 0;
pub const STATE_RECORDING: u8 = 1;
pub const STATE_TRANSCRIBING: u8 = 2;

/// Shared application state managed by Tauri
pub struct AppState {
    pub recording_state: AtomicU8,
    pub audio_buffer: Arc<Mutex<Vec<f32>>>,
    pub audio_stop_flag: Arc<AtomicBool>,
    pub audio_sample_rate: Arc<AtomicU32>,
    pub audio_thread: Mutex<Option<std::thread::JoinHandle<()>>>,
    pub whisper_engine: Mutex<WhisperEngine>,
    pub db: Mutex<Option<Connection>>,
    pub config: RwLock<Settings>,
    pub data_dir: PathBuf,
}

impl AppState {
    pub fn new() -> Self {
        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("com.g1tech.voice2txt");

        std::fs::create_dir_all(&data_dir).ok();

        let config = Settings::load(&data_dir).unwrap_or_default();

        Self {
            recording_state: AtomicU8::new(STATE_IDLE),
            audio_buffer: Arc::new(Mutex::new(Vec::new())),
            audio_stop_flag: Arc::new(AtomicBool::new(false)),
            audio_sample_rate: Arc::new(AtomicU32::new(0)),
            audio_thread: Mutex::new(None),
            whisper_engine: Mutex::new(WhisperEngine::new()),
            db: Mutex::new(None),
            config: RwLock::new(config),
            data_dir,
        }
    }

    pub fn get_state(&self) -> &'static str {
        match self.recording_state.load(Ordering::Relaxed) {
            STATE_RECORDING => "recording",
            STATE_TRANSCRIBING => "transcribing",
            _ => "idle",
        }
    }

    pub fn set_state(&self, state: u8) {
        self.recording_state.store(state, Ordering::Relaxed);
    }

    pub fn init_db(&self) -> Result<(), Box<dyn std::error::Error>> {
        let db_path = self.data_dir.join("voice2txt.db");
        let conn = Connection::open(db_path)?;
        crate::db::migrations::run_migrations(&conn)?;
        *self.db.lock() = Some(conn);
        Ok(())
    }
}
