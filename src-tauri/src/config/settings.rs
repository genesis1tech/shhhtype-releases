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
    /// Seconds of silence before auto-stopping recording.
    #[serde(default = "default_vad_silence_timeout")]
    pub vad_silence_timeout: f32,
    /// Show the floating overlay during recording.
    pub show_overlay: bool,
    /// Play sounds on start/stop recording.
    pub sound_feedback: bool,
    /// Launch at login.
    #[serde(default)]
    pub auto_launch: bool,
    /// Transcription backend: local whisper or cloud (Groq).
    #[serde(default)]
    pub transcription_backend: TranscriptionBackend,
    /// Groq API key for cloud transcription.
    #[serde(default)]
    pub groq_api_key: Option<String>,
    /// Enable AI rewrite after transcription.
    #[serde(default)]
    pub rewrite_enabled: bool,
    /// Style for AI rewrite.
    #[serde(default)]
    pub rewrite_style: RewriteStyle,
    /// Hotkey for AI rewrite (e.g., "CmdOrCtrl+Alt+R").
    #[serde(default = "default_rewrite_hotkey")]
    pub rewrite_hotkey: String,
}

fn default_vad_silence_timeout() -> f32 {
    15.0
}

fn default_rewrite_hotkey() -> String {
    "CmdOrCtrl+Alt+R".to_string()
}

/// Transcription backend selection.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub enum TranscriptionBackend {
    #[default]
    Local,
    Cloud,
}

/// AI rewrite style.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub enum RewriteStyle {
    #[default]
    Professional,
    Casual,
    Concise,
    Friendly,
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
            vad_silence_timeout: 15.0,
            show_overlay: true,
            sound_feedback: true,
            auto_launch: false,
            transcription_backend: TranscriptionBackend::Cloud,
            groq_api_key: None,
            rewrite_enabled: false,
            rewrite_style: RewriteStyle::Professional,
            rewrite_hotkey: default_rewrite_hotkey(),
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
