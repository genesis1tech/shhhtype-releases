/// Energy-based Voice Activity Detection using RMS level.
/// Simple but effective for push-to-talk scenarios.
pub struct EnergyVad {
    /// RMS threshold below which audio is considered silence.
    pub threshold: f32,
    /// Minimum consecutive silent frames before stopping.
    pub silence_frames: usize,
    current_silence: usize,
}

impl EnergyVad {
    pub fn new(threshold: f32, silence_frames: usize) -> Self {
        Self {
            threshold,
            silence_frames,
            current_silence: 0,
        }
    }

    /// Calculate RMS energy of audio samples.
    pub fn rms(samples: &[f32]) -> f32 {
        if samples.is_empty() {
            return 0.0;
        }
        let sum_sq: f32 = samples.iter().map(|s| s * s).sum();
        (sum_sq / samples.len() as f32).sqrt()
    }

    /// Check if the given audio chunk contains speech.
    pub fn is_speech(&mut self, samples: &[f32]) -> bool {
        let energy = Self::rms(samples);
        if energy >= self.threshold {
            self.current_silence = 0;
            true
        } else {
            self.current_silence += 1;
            self.current_silence < self.silence_frames
        }
    }

    /// Reset the VAD state.
    pub fn reset(&mut self) {
        self.current_silence = 0;
    }
}

impl Default for EnergyVad {
    fn default() -> Self {
        Self::new(0.01, 30) // ~30 frames of silence at typical chunk sizes
    }
}
