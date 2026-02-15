use anyhow::Result;
use std::path::{Path, PathBuf};

/// Available Whisper model sizes.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum ModelSize {
    Tiny,
    Base,
    Small,
    Medium,
    LargeV3,
    LargeV3Turbo,
}

impl ModelSize {
    /// Get the filename for this model size.
    pub fn filename(&self) -> &str {
        match self {
            ModelSize::Tiny => "ggml-tiny.bin",
            ModelSize::Base => "ggml-base.bin",
            ModelSize::Small => "ggml-small.bin",
            ModelSize::Medium => "ggml-medium.bin",
            ModelSize::LargeV3 => "ggml-large-v3.bin",
            ModelSize::LargeV3Turbo => "ggml-large-v3-turbo.bin",
        }
    }

    /// Human-readable display name.
    pub fn display_name(&self) -> &str {
        match self {
            ModelSize::Tiny => "Tiny (75MB)",
            ModelSize::Base => "Base (142MB)",
            ModelSize::Small => "Small (466MB)",
            ModelSize::Medium => "Medium (1.5GB)",
            ModelSize::LargeV3 => "Large V3 (3.1GB)",
            ModelSize::LargeV3Turbo => "Large V3 Turbo (1.6GB)",
        }
    }
}

impl Default for ModelSize {
    fn default() -> Self {
        ModelSize::Base
    }
}

/// Resolve the full path for a model within the data directory.
pub fn model_path(data_dir: &Path, size: &ModelSize) -> PathBuf {
    data_dir.join("models").join(size.filename())
}

/// Check if a model file exists locally.
pub fn is_model_downloaded(data_dir: &Path, size: &ModelSize) -> bool {
    model_path(data_dir, size).exists()
}

/// Ensure the models directory exists.
pub fn ensure_models_dir(data_dir: &Path) -> Result<()> {
    let dir = data_dir.join("models");
    std::fs::create_dir_all(&dir)?;
    Ok(())
}

/// List all locally available models.
pub fn list_downloaded_models(data_dir: &Path) -> Vec<ModelSize> {
    let all = vec![
        ModelSize::Tiny,
        ModelSize::Base,
        ModelSize::Small,
        ModelSize::Medium,
        ModelSize::LargeV3,
        ModelSize::LargeV3Turbo,
    ];
    all.into_iter()
        .filter(|s| is_model_downloaded(data_dir, s))
        .collect()
}
