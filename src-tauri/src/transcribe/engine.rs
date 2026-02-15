use anyhow::Result;
use std::path::Path;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

/// Whisper transcription engine wrapping whisper-rs.
pub struct WhisperEngine {
    context: Option<WhisperContext>,
}

impl WhisperEngine {
    pub fn new() -> Self {
        Self { context: None }
    }

    /// Load a Whisper model from disk.
    pub fn load_model(&mut self, model_path: &Path) -> Result<()> {
        if !model_path.exists() {
            anyhow::bail!("Model not found: {}", model_path.display());
        }

        let params = WhisperContextParameters::default();
        let ctx = WhisperContext::new_with_params(
            model_path.to_str().ok_or_else(|| anyhow::anyhow!("Invalid model path"))?,
            params,
        )
        .map_err(|e| anyhow::anyhow!("Failed to load whisper model: {}", e))?;

        self.context = Some(ctx);
        log::info!("Whisper model loaded: {}", model_path.display());
        Ok(())
    }

    /// Transcribe 16kHz mono f32 audio samples to text.
    pub fn transcribe(&self, samples: &[f32]) -> Result<String> {
        let ctx = self
            .context
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No model loaded"))?;

        let mut state = ctx.create_state().map_err(|e| anyhow::anyhow!("Failed to create whisper state: {}", e))?;

        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_language(Some("en"));
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        params.set_no_context(true);
        params.set_single_segment(false);

        state
            .full(params, samples)
            .map_err(|e| anyhow::anyhow!("Whisper inference failed: {}", e))?;

        let num_segments = state
            .full_n_segments()
            .map_err(|e| anyhow::anyhow!("Failed to get segments: {}", e))?;

        let mut text = String::new();
        for i in 0..num_segments {
            let segment = state
                .full_get_segment_text(i)
                .map_err(|e| anyhow::anyhow!("Failed to get segment text: {}", e))?;
            text.push_str(&segment);
        }

        Ok(text.trim().to_string())
    }

    /// Check if a model is loaded and ready.
    pub fn is_loaded(&self) -> bool {
        self.context.is_some()
    }
}
