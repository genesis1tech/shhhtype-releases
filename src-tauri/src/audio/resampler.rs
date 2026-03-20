use anyhow::Result;
use rubato::{SincFixedIn, SincInterpolationParameters, SincInterpolationType, WindowFunction, Resampler};

/// Resample audio from source sample rate to 16kHz (Whisper's required rate).
pub fn resample_to_16khz(samples: &[f32], from_rate: u32) -> Result<Vec<f32>> {
    if from_rate == 16000 {
        return Ok(samples.to_vec());
    }

    let params = SincInterpolationParameters {
        sinc_len: 64,
        f_cutoff: 0.95,
        interpolation: SincInterpolationType::Linear,
        oversampling_factor: 128,
        window: WindowFunction::BlackmanHarris2,
    };

    let ratio = 16000.0 / from_rate as f64;
    let mut resampler = SincFixedIn::<f64>::new(
        ratio,
        2.0,   // max relative ratio
        params,
        samples.len(),
        1, // mono
    )?;

    // Convert f32 -> f64 for rubato
    let input_f64: Vec<f64> = samples.iter().map(|&s| s as f64).collect();
    let result = resampler.process(&[input_f64], None)?;

    // Convert f64 -> f32
    let output: Vec<f32> = result[0].iter().map(|&s| s as f32).collect();

    Ok(output)
}
