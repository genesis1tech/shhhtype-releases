use crate::config::settings::RewriteStyle;
use crate::state::GroqUsage;
use anyhow::{anyhow, Result};
use parking_lot::Mutex;
use serde::Deserialize;

const GROQ_CHAT_URL: &str = "https://api.groq.com/openai/v1/chat/completions";

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

#[derive(Deserialize)]
struct ChatMessage {
    content: String,
}

fn system_prompt(style: &RewriteStyle) -> &'static str {
    match style {
        RewriteStyle::Professional => {
            "Rewrite this to be professional and polished. Preserve the meaning and tone. Do not add information. Return only the rewritten text."
        }
        RewriteStyle::Casual => {
            "Rewrite this to be natural and conversational. Keep it concise. Return only the rewritten text."
        }
        RewriteStyle::Concise => {
            "Make this shorter and more direct. Remove filler words. Preserve meaning. Return only the rewritten text."
        }
        RewriteStyle::Friendly => {
            "Rewrite this to be warm and approachable. Keep it genuine. Return only the rewritten text."
        }
    }
}

/// Rewrite text using Groq's Llama 3.3 70B model.
pub fn rewrite_text(text: &str, style: &RewriteStyle, api_key: &str, usage: Option<&Mutex<GroqUsage>>) -> Result<String> {
    let body = serde_json::json!({
        "model": "llama-3.3-70b-versatile",
        "messages": [
            { "role": "system", "content": system_prompt(style) },
            { "role": "user", "content": text }
        ],
        "temperature": 0.3,
        "max_tokens": 2048,
    });

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let resp = client
        .post(GROQ_CHAT_URL)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().unwrap_or_default();
        return Err(anyhow!("Groq API error {}: {}", status, body));
    }

    if let Some(usage) = usage {
        crate::state::update_groq_usage(resp.headers(), usage);
    }

    let response: ChatResponse = resp.json()?;
    let content = response
        .choices
        .first()
        .map(|c| c.message.content.trim().to_string())
        .unwrap_or_default();

    if content.is_empty() {
        return Err(anyhow!("Empty response from Groq"));
    }

    Ok(content)
}
