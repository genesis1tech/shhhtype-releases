use crate::hotkey::manager::HotkeyMode;
use crate::transcribe::model::ModelSize;
use anyhow::Result;
use std::path::Path;

/// Application settings persisted to JSON.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    /// Which Whisper model to use.
    pub model_size: ModelSize,
    /// Global shortcut string (e.g., "CmdOrCtrl+Shift+Space").
    pub shortcut: String,
    /// Push-to-talk vs toggle mode.
    pub hotkey_mode: HotkeyMode,
    /// Text injection method.
    pub injection_method: InjectionMethod,
    /// Language for transcription (e.g., "en").
    pub language: String,
    /// Whether to auto-copy transcription to clipboard.
    pub auto_copy: bool,
    /// VAD silence threshold (RMS).
    pub vad_threshold: f32,
    /// Show the floating overlay during recording.
    pub show_overlay: bool,
    /// Play sounds on start/stop recording.
    pub sound_feedback: bool,
    /// Launch at login.
    #[serde(default)]
    pub auto_launch: bool,
    /// Where transcription runs: Local (on-device) or Groq (cloud API).
    #[serde(default)]
    pub transcription_backend: TranscriptionBackend,
    /// Groq API key (only used when backend = Groq).
    #[serde(default)]
    pub groq_api_key: Option<String>,
}

/// Where transcription runs.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub enum TranscriptionBackend {
    /// Run Whisper locally on-device (default).
    #[default]
    Local,
    /// Send audio to Groq's free Whisper API (whisper-large-v3-turbo).
    Groq,
}

/// How to inject transcribed text.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum InjectionMethod {
    /// Clipboard paste (Cmd+V) - most compatible.
    Clipboard,
    /// Character-by-character keyboard simulation.
    Keyboard,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            model_size: ModelSize::Base,
            shortcut: "CmdOrCtrl+Alt+V".to_string(),
            hotkey_mode: HotkeyMode::PushToTalk,
            injection_method: InjectionMethod::Clipboard,
            language: "en".to_string(),
            auto_copy: false,
            vad_threshold: 0.01,
            show_overlay: true,
            sound_feedback: true,
            auto_launch: false,
            transcription_backend: TranscriptionBackend::Local,
            groq_api_key: None,
        }
    }
}

impl Settings {
    /// Load settings from the data directory.
    pub fn load(data_dir: &Path) -> Result<Self> {
        let path = data_dir.join("settings.json");
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(&path)?;
        let settings: Settings = serde_json::from_str(&content)?;
        Ok(settings)
    }

    /// Save settings to the data directory.
    pub fn save(&self, data_dir: &Path) -> Result<()> {
        let path = data_dir.join("settings.json");
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, content)?;
        Ok(())
    }
}
