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
    /// Selected audio input device name. None = system default.
    #[serde(default)]
    pub audio_input_device: Option<String>,
    /// Where to show the recording overlay.
    #[serde(default)]
    pub overlay_position: OverlayPosition,
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

/// Where to show the recording overlay.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub enum OverlayPosition {
    #[default]
    TopCenter,
    Inline,
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
            audio_input_device: None,
            overlay_position: OverlayPosition::TopCenter,
        }
    }
}

const KEYCHAIN_GROQ_API_KEY: &str = "groq_api_key";

impl Settings {
    /// Load settings from the data directory.
    /// Retrieves secrets from the OS keychain instead of plaintext JSON.
    pub fn load(data_dir: &Path) -> Result<Self> {
        let path = data_dir.join("settings.json");
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(&path)?;
        let mut settings: Settings = serde_json::from_str(&content)?;

        // Retrieve API key from keychain (overrides any value in JSON)
        if let Some(key) = super::keychain::get_secret(KEYCHAIN_GROQ_API_KEY) {
            settings.groq_api_key = Some(key);
        }

        // Migrate: if a plaintext API key exists in settings.json, move it to keychain
        if settings.groq_api_key.is_some() {
            // Re-read the raw JSON to check if the key was stored in plaintext
            if let Ok(raw) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(plain_key) = raw.get("groq_api_key").and_then(|v| v.as_str()) {
                    if !plain_key.is_empty() {
                        log::info!("Migrating groq_api_key from plaintext settings to keychain");
                        if super::keychain::set_secret(KEYCHAIN_GROQ_API_KEY, plain_key).is_ok() {
                            // Re-save settings without the plaintext key
                            let mut clean = settings.clone();
                            clean.groq_api_key = settings.groq_api_key.clone();
                            clean.save_without_secrets(data_dir)?;
                        }
                    }
                }
            }
        }

        Ok(settings)
    }

    /// Save settings to the data directory.
    /// Stores secrets in the OS keychain and excludes them from the JSON file.
    pub fn save(&self, data_dir: &Path) -> Result<()> {
        // Store API key in keychain
        if let Some(ref key) = self.groq_api_key {
            if !key.is_empty() {
                if let Err(e) = super::keychain::set_secret(KEYCHAIN_GROQ_API_KEY, key) {
                    log::error!("Failed to store API key in keychain: {}", e);
                }
            }
        } else {
            let _ = super::keychain::delete_secret(KEYCHAIN_GROQ_API_KEY);
        }

        self.save_without_secrets(data_dir)
    }

    /// Save settings JSON without embedding secrets in plaintext.
    fn save_without_secrets(&self, data_dir: &Path) -> Result<()> {
        let path = data_dir.join("settings.json");
        let mut clean = self.clone();
        // Never write the API key to disk — it lives in the keychain
        clean.groq_api_key = None;
        let content = serde_json::to_string_pretty(&clean)?;
        std::fs::write(&path, content)?;
        Ok(())
    }
}
