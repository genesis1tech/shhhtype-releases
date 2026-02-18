use anyhow::{anyhow, Result};
use hound::{SampleFormat, WavSpec, WavWriter};
use serde::Deserialize;
use std::io::Cursor;

const GROQ_API_URL: &str = "https://api.groq.com/openai/v1/audio/transcriptions";

#[derive(Deserialize)]
struct GroqResponse {
    text: String,
}

/// Encode f32 PCM samples (16kHz mono) to WAV bytes in memory.
fn encode_wav(samples: &[f32]) -> Result<Vec<u8>> {
    let spec = WavSpec {
        channels: 1,
        sample_rate: 16000,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
    };
    let mut cursor = Cursor::new(Vec::new());
    {
        let mut writer = WavWriter::new(&mut cursor, spec)?;
        for &s in samples {
            writer.write_sample(s)?;
        }
        writer.finalize()?;
    }
    Ok(cursor.into_inner())
}

/// Send audio to Groq's Whisper API and return transcribed text.
/// Uses whisper-large-v3-turbo (best speed/accuracy balance on Groq).
pub fn transcribe(samples: &[f32], language: &str, api_key: &str) -> Result<String> {
    let wav_bytes = encode_wav(samples)?;

    let part = reqwest::blocking::multipart::Part::bytes(wav_bytes)
        .file_name("audio.wav")
        .mime_str("audio/wav")
        .map_err(|e| anyhow!("Invalid MIME type: {}", e))?;

    let lang = if language == "auto" {
        // Groq doesn't support "auto" — omit language field for auto-detection
        None
    } else {
        Some(language.to_string())
    };

    let mut form = reqwest::blocking::multipart::Form::new()
        .part("file", part)
        .text("model", "whisper-large-v3-turbo")
        .text("response_format", "json");

    if let Some(l) = lang {
        form = form.text("language", l);
    }

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let resp = client
        .post(GROQ_API_URL)
        .header("Authorization", format!("Bearer {}", api_key))
        .multipart(form)
        .send()?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().unwrap_or_default();
        return Err(anyhow!("Groq API error {}: {}", status, body));
    }

    let response: GroqResponse = resp.json()?;
    Ok(response.text.trim().to_string())
}
