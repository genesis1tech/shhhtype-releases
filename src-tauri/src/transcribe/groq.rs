use crate::state::GroqUsage;
use anyhow::{anyhow, Result};
use hound::{SampleFormat, WavSpec, WavWriter};
use parking_lot::Mutex;
use serde::Deserialize;
use std::io::Cursor;
use std::sync::LazyLock;

const GROQ_API_URL: &str = "https://api.groq.com/openai/v1/audio/transcriptions";

/// Reuse a single HTTP client across requests (saves TLS handshake).
static HTTP_CLIENT: LazyLock<reqwest::blocking::Client> = LazyLock::new(|| {
    reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .expect("Failed to build HTTP client")
});

#[derive(Deserialize)]
struct GroqResponse {
    text: String,
}

/// Encode f32 PCM samples to 16-bit WAV bytes in memory.
/// Accepts any sample rate — Groq handles downsampling server-side.
fn encode_wav(samples: &[f32], sample_rate: u32) -> Result<Vec<u8>> {
    let spec = WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    let mut cursor = Cursor::new(Vec::with_capacity(samples.len() * 2 + 44));
    {
        let mut writer = WavWriter::new(&mut cursor, spec)?;
        for &s in samples {
            let clamped = s.clamp(-1.0, 1.0);
            let i16_val = (clamped * 32767.0) as i16;
            writer.write_sample(i16_val)?;
        }
        writer.finalize()?;
    }
    Ok(cursor.into_inner())
}

/// Send audio to Groq's Whisper API and return transcribed text.
/// Uses distil-whisper for English (faster, cheaper) or whisper-large-v3-turbo for other languages.
pub fn transcribe(
    samples: &[f32],
    sample_rate: u32,
    language: &str,
    api_key: &str,
    usage: Option<&Mutex<GroqUsage>>,
) -> Result<String> {
    let wav_bytes = encode_wav(samples, sample_rate)?;
    log::info!("WAV encoded: {} bytes ({} samples at {}Hz)", wav_bytes.len(), samples.len(), sample_rate);

    let part = reqwest::blocking::multipart::Part::bytes(wav_bytes)
        .file_name("audio.wav")
        .mime_str("audio/wav")
        .map_err(|e| anyhow!("Invalid MIME type: {}", e))?;

    // whisper-large-v3-turbo: best speed/accuracy balance on Groq
    let model = "whisper-large-v3-turbo";

    let lang = if language == "auto" {
        None
    } else {
        Some(language.to_string())
    };

    let mut form = reqwest::blocking::multipart::Form::new()
        .part("file", part)
        .text("model", model)
        .text("response_format", "json");

    if let Some(l) = lang {
        form = form.text("language", l);
    }

    log::info!("Sending to Groq (model: {})...", model);

    let resp = HTTP_CLIENT
        .post(GROQ_API_URL)
        .header("Authorization", format!("Bearer {}", api_key))
        .multipart(form)
        .send()?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().unwrap_or_default();
        return Err(anyhow!("Groq API error {}: {}", status, body));
    }

    if let Some(usage) = usage {
        crate::state::update_groq_usage(resp.headers(), usage);
    }

    let response: GroqResponse = resp.json()?;
    Ok(response.text.trim().to_string())
}
