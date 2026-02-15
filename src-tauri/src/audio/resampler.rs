use anyhow::Result;

/// Resample audio from source sample rate to 16kHz (Whisper's required rate).
pub fn resample_to_16khz(samples: &[f32], from_rate: u32) -> Result<Vec<f32>> {
    if from_rate == 16000 {
        return Ok(samples.to_vec());
    }

    // Use linear interpolation for simplicity in MVP.
    // TODO: Use rubato for higher-quality resampling.
    let ratio = 16000.0 / from_rate as f64;
    let output_len = (samples.len() as f64 * ratio) as usize;
    let mut output = Vec::with_capacity(output_len);

    for i in 0..output_len {
        let src_idx = i as f64 / ratio;
        let idx = src_idx as usize;
        let frac = src_idx - idx as f64;

        if idx + 1 < samples.len() {
            let sample = samples[idx] as f64 * (1.0 - frac) + samples[idx + 1] as f64 * frac;
            output.push(sample as f32);
        } else if idx < samples.len() {
            output.push(samples[idx]);
        }
    }

    Ok(output)
}
