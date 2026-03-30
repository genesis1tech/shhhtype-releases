/// Audio normalization and compression for improving transcription accuracy.
/// Boosts quiet recordings (distant/poor microphones) to optimal levels.

/// Peak-normalize audio samples so the loudest sample reaches the target level.
///
/// Safety guards:
/// - Skips if peak is below noise floor (avoids amplifying silence)
/// - Caps gain at `max_gain` to prevent extreme amplification
fn normalize_peak(samples: &mut [f32], target_peak: f32) {
    const NOISE_FLOOR: f32 = 0.001;
    const MAX_GAIN: f32 = 40.0; // ~32 dB max boost

    let peak = samples
        .iter()
        .map(|s| s.abs())
        .fold(0.0f32, f32::max);

    if peak < NOISE_FLOOR {
        log::debug!("Audio peak {:.6} below noise floor, skipping normalization", peak);
        return;
    }

    let gain = (target_peak / peak).min(MAX_GAIN);
    log::info!(
        "Normalizing audio: peak={:.4}, target={:.2}, gain={:.2}x ({:.1} dB)",
        peak,
        target_peak,
        gain,
        20.0 * gain.log10()
    );

    for sample in samples.iter_mut() {
        *sample *= gain;
    }
}

/// Soft-knee compressor: tames peaks above threshold with the given ratio.
///
/// For voice, a 4:1 ratio above 0.5 threshold keeps dynamics natural
/// while preventing clipping after normalization.
fn soft_compress(samples: &mut [f32], threshold: f32, ratio: f32) {
    for sample in samples.iter_mut() {
        let abs_val = sample.abs();
        if abs_val > threshold {
            let excess = abs_val - threshold;
            let compressed = threshold + excess / ratio;
            *sample = compressed.copysign(*sample);
        }
    }
}

/// Normalize and compress audio for optimal transcription.
///
/// 1. Peak-normalizes to 0.9 (leaves headroom below clipping)
/// 2. Applies gentle 4:1 compression above 0.5 threshold
pub fn normalize_audio(samples: &mut [f32]) {
    normalize_peak(samples, 0.9);
    soft_compress(samples, 0.5, 4.0);
}
