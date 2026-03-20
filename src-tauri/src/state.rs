use crate::config::settings::{InjectionMethod, Settings};
use crate::transcribe::engine::WhisperEngine;
use parking_lot::{Mutex, RwLock};
use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU8, Ordering};
use std::sync::Arc;
use std::time::Instant;

/// Rate limit usage data extracted from Groq API response headers.
#[derive(Clone, Default, serde::Serialize)]
pub struct GroqUsage {
    pub limit_requests: Option<u32>,
    pub remaining_requests: Option<u32>,
    pub reset_requests: Option<String>,
    pub limit_tokens: Option<u32>,
    pub remaining_tokens: Option<u32>,
    pub reset_tokens: Option<String>,
    pub updated_at: Option<String>,
}

/// Extract Groq rate limit headers from a response and update shared usage state.
pub fn update_groq_usage(
    headers: &reqwest::header::HeaderMap,
    usage: &Mutex<GroqUsage>,
) {
    let get = |name: &str| -> Option<String> {
        headers.get(name).and_then(|v| v.to_str().ok()).map(|s| s.to_string())
    };
    let get_u32 = |name: &str| -> Option<u32> {
        get(name).and_then(|s| s.parse().ok())
    };

    let mut u = usage.lock();
    if let Some(v) = get_u32("x-ratelimit-limit-requests") { u.limit_requests = Some(v); }
    if let Some(v) = get_u32("x-ratelimit-remaining-requests") { u.remaining_requests = Some(v); }
    if let Some(v) = get("x-ratelimit-reset-requests") { u.reset_requests = Some(v); }
    if let Some(v) = get_u32("x-ratelimit-limit-tokens") { u.limit_tokens = Some(v); }
    if let Some(v) = get_u32("x-ratelimit-remaining-tokens") { u.remaining_tokens = Some(v); }
    if let Some(v) = get("x-ratelimit-reset-tokens") { u.reset_tokens = Some(v); }
    u.updated_at = Some(chrono::Utc::now().to_rfc3339());
}

/// Composition buffer TTL in seconds (30 minutes).
const BUFFER_TTL_SECS: u64 = 1800;
/// Maximum entries in the composition buffer.
const BUFFER_MAX_ENTRIES: usize = 20;

/// A single transcription segment with metadata for rewrite replacement.
pub struct CompositionSegment {
    pub text: String,
    pub char_count: usize,
    pub injection_method: InjectionMethod,
}

/// Accumulates transcription segments for multi-segment rewrite.
pub struct CompositionBuffer {
    entries: Vec<CompositionSegment>,
    /// Total characters injected across all segments (for selection-based replacement on rewrite).
    injected_chars: usize,
    last_appended_at: Option<Instant>,
}

impl CompositionBuffer {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            injected_chars: 0,
            last_appended_at: None,
        }
    }

    /// Append a transcription segment. Clears stale entries if TTL expired, drops oldest if at max cap.
    pub fn append(&mut self, text: String, injection_method: InjectionMethod) {
        // Clear if TTL expired
        if let Some(last) = self.last_appended_at {
            if last.elapsed().as_secs() >= BUFFER_TTL_SECS {
                self.entries.clear();
                self.injected_chars = 0;
            }
        }
        // Drop oldest if at max
        if self.entries.len() >= BUFFER_MAX_ENTRIES {
            let removed = self.entries.remove(0);
            self.injected_chars = self.injected_chars.saturating_sub(removed.char_count);
        }
        let char_count = text.chars().count();
        self.injected_chars += char_count;
        self.entries.push(CompositionSegment { text, char_count, injection_method });
        self.last_appended_at = Some(Instant::now());
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.injected_chars = 0;
        self.last_appended_at = None;
    }

    /// Join all entry texts with a space separator.
    pub fn join(&self) -> String {
        self.entries.iter().map(|s| s.text.as_str()).collect::<Vec<_>>().join(" ")
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Total characters injected across all buffered segments.
    pub fn injected_chars(&self) -> usize {
        self.injected_chars
    }

    /// True when there are 2+ segments in the buffer.
    pub fn is_multi(&self) -> bool {
        self.entries.len() > 1
    }
}

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
    /// When recording started, for duration tracking.
    pub recording_started_at: Mutex<Option<Instant>>,
    /// Last transcription text, used by AI rewrite.
    pub last_transcription: Mutex<Option<String>>,
    /// Groq API rate limit usage, updated after each API call.
    pub groq_usage: Mutex<GroqUsage>,
    /// Composition buffer for multi-segment rewrite.
    pub composition: Mutex<CompositionBuffer>,
    /// Cached dictionary to avoid disk I/O on every transcription.
    pub dictionary_cache: Mutex<Option<crate::transcribe::dictionary::Dictionary>>,
    /// Generation counter for overlay visibility — stale hide timers check this
    /// to avoid hiding the overlay when a new action (recording, rewrite) has started.
    pub overlay_generation: AtomicU32,
}

impl AppState {
    pub fn new() -> Self {
        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("com.g1tech.shhhtype");

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
            recording_started_at: Mutex::new(None),
            last_transcription: Mutex::new(None),
            groq_usage: Mutex::new(GroqUsage::default()),
            composition: Mutex::new(CompositionBuffer::new()),
            dictionary_cache: Mutex::new(None),
            overlay_generation: AtomicU32::new(0),
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
        let db_path = self.data_dir.join("shhhtype.db");
        let conn = Connection::open(db_path)?;
        crate::db::migrations::run_migrations(&conn)?;
        *self.db.lock() = Some(conn);
        Ok(())
    }
}
