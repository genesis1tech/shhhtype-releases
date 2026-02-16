use anyhow::{bail, Result};
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

    /// Hugging Face download URL for this model.
    pub fn download_url(&self) -> String {
        format!(
            "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/{}",
            self.filename()
        )
    }

    /// Expected SHA-256 hash for integrity verification after download.
    pub fn expected_sha256(&self) -> &str {
        match self {
            ModelSize::Tiny => "be07e048e1e599ad46341c8d2a135645097a538221678b7acdd1b1919c6e1b21",
            ModelSize::Base => "60ed5bc3dd14eea856493d334349b405782ddcaf0028d4b5df4088345fba2efe",
            // Hashes for larger models verified on first download and logged
            ModelSize::Small => "0000000000000000000000000000000000000000000000000000000000000000",
            ModelSize::Medium => "0000000000000000000000000000000000000000000000000000000000000000",
            ModelSize::LargeV3 => "0000000000000000000000000000000000000000000000000000000000000000",
            ModelSize::LargeV3Turbo => "0000000000000000000000000000000000000000000000000000000000000000",
        }
    }

    /// String identifier for events/serialization.
    pub fn id(&self) -> &str {
        match self {
            ModelSize::Tiny => "Tiny",
            ModelSize::Base => "Base",
            ModelSize::Small => "Small",
            ModelSize::Medium => "Medium",
            ModelSize::LargeV3 => "LargeV3",
            ModelSize::LargeV3Turbo => "LargeV3Turbo",
        }
    }
}

impl Default for ModelSize {
    fn default() -> Self {
        ModelSize::Base
    }
}

/// Status of a single model on disk.
#[derive(serde::Serialize, Clone)]
pub struct ModelStatus {
    pub model: String,
    pub downloaded: bool,
    pub size_bytes: Option<u64>,
}

/// Download progress event payload.
#[derive(serde::Serialize, Clone)]
pub struct DownloadProgress {
    pub model: String,
    pub downloaded: u64,
    pub total: u64,
    pub percent: f64,
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

/// Get download status of all models.
pub fn get_all_model_status(data_dir: &Path) -> Vec<ModelStatus> {
    let all = vec![
        ModelSize::Tiny,
        ModelSize::Base,
        ModelSize::Small,
        ModelSize::Medium,
        ModelSize::LargeV3,
        ModelSize::LargeV3Turbo,
    ];
    all.iter()
        .map(|size| {
            let path = model_path(data_dir, size);
            let downloaded = path.exists();
            let size_bytes = if downloaded {
                std::fs::metadata(&path).ok().map(|m| m.len())
            } else {
                None
            };
            ModelStatus {
                model: size.id().to_string(),
                downloaded,
                size_bytes,
            }
        })
        .collect()
}

/// Download a model from Hugging Face with progress events.
pub async fn download_model(
    app: tauri::AppHandle,
    data_dir: PathBuf,
    size: ModelSize,
) -> Result<()> {
    use futures_util::StreamExt;
    use tauri::Emitter;

    ensure_models_dir(&data_dir)?;

    let url = size.download_url();
    let dest = model_path(&data_dir, &size);
    let tmp_path = dest.with_extension("bin.tmp");

    log::info!("Downloading model {} from {}", size.id(), url);

    let response = reqwest::get(&url).await?;
    let total = response.content_length().unwrap_or(0);

    let mut file = tokio::fs::File::create(&tmp_path).await?;
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        tokio::io::AsyncWriteExt::write_all(&mut file, &chunk).await?;
        downloaded += chunk.len() as u64;

        let percent = if total > 0 {
            (downloaded as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        let _ = app.emit(
            "model-download-progress",
            DownloadProgress {
                model: size.id().to_string(),
                downloaded,
                total,
                percent,
            },
        );
    }

    // Verify SHA-256 integrity before accepting the file
    drop(file); // ensure all writes are flushed
    let expected = size.expected_sha256();
    let is_placeholder = expected.chars().all(|c| c == '0');

    let tmp_path_clone = tmp_path.clone();
    let computed = tokio::task::spawn_blocking(move || -> Result<String> {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        let mut f = std::fs::File::open(&tmp_path_clone)?;
        std::io::copy(&mut f, &mut hasher)?;
        Ok(format!("{:x}", hasher.finalize()))
    })
    .await??;

    if is_placeholder {
        log::info!(
            "Model {} SHA-256: {} (no known hash to verify — save this for future builds)",
            size.id(),
            computed
        );
    } else if computed != expected {
        // Delete the corrupt file and bail
        let _ = tokio::fs::remove_file(&tmp_path).await;
        bail!(
            "SHA-256 mismatch for model {}: expected {} but got {}",
            size.id(),
            expected,
            computed
        );
    } else {
        log::info!("Model {} SHA-256 verified: {}", size.id(), computed);
    }

    // Atomic rename from .tmp to final path
    tokio::fs::rename(&tmp_path, &dest).await?;
    log::info!("Model {} downloaded successfully", size.id());

    Ok(())
}
