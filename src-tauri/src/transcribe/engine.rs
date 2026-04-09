use anyhow::Result;
use std::path::Path;
use std::time::Duration;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

/// Maximum time to wait for Whisper inference before timing out.
/// Metal GPU stalls or pathological audio can cause indefinite hangs.
const TRANSCRIBE_TIMEOUT: Duration = Duration::from_secs(60);

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
    /// Runs inference on a dedicated thread with a timeout to prevent indefinite hangs
    /// from Metal GPU stalls or pathological audio inputs.
    pub fn transcribe(&self, samples: &[f32], language: &str) -> Result<String> {
        let ctx = self
            .context
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No model loaded"))?;

        let mut state = ctx.create_state().map_err(|e| anyhow::anyhow!("Failed to create whisper state: {}", e))?;

        // Run inference on a separate thread with a timeout so a Metal GPU stall
        // or pathological audio doesn't hang the app forever.
        let samples_owned = samples.to_vec();
        let language_owned = language.to_string();
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            // Build params inside the thread so the language borrow is owned here
            let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
            params.set_language(Some(&language_owned));
            params.set_print_progress(false);
            params.set_print_realtime(false);
            params.set_print_timestamps(false);
            params.set_no_context(true);
            params.set_single_segment(false);

            let result = state
                .full(params, &samples_owned)
                .map_err(|e| anyhow::anyhow!("Whisper inference failed: {}", e));

            if let Err(ref e) = result {
                let _ = tx.send(Err(anyhow::anyhow!("{}", e)));
                return;
            }

            let num_segments = match state.full_n_segments() {
                Ok(n) => n,
                Err(e) => {
                    let _ = tx.send(Err(anyhow::anyhow!("Failed to get segments: {}", e)));
                    return;
                }
            };

            let mut text = String::new();
            for i in 0..num_segments {
                match state.full_get_segment_text(i) {
                    Ok(segment) => text.push_str(&segment),
                    Err(e) => {
                        let _ = tx.send(Err(anyhow::anyhow!("Failed to get segment text: {}", e)));
                        return;
                    }
                }
            }

            let _ = tx.send(Ok(text.trim().to_string()));
        });

        match rx.recv_timeout(TRANSCRIBE_TIMEOUT) {
            Ok(result) => result,
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                log::error!("Whisper inference timed out after {:?}", TRANSCRIBE_TIMEOUT);
                Err(anyhow::anyhow!(
                    "Transcription timed out after {}s — try a shorter recording or switch to cloud transcription",
                    TRANSCRIBE_TIMEOUT.as_secs()
                ))
            }
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                log::error!("Whisper inference thread panicked");
                Err(anyhow::anyhow!("Transcription thread crashed unexpectedly"))
            }
        }
    }

    /// Check if a model is loaded and ready.
    pub fn is_loaded(&self) -> bool {
        self.context.is_some()
    }
}
