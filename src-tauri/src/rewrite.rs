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

/// Strip `<think>...</think>` blocks from model responses (e.g. Qwen3 chain-of-thought).
fn strip_think_tags(text: &str) -> &str {
    if let Some(end) = text.find("</think>") {
        text[end + 8..].trim_start()
    } else {
        text
    }
}

/// Convert a single ASCII char to its Unicode Mathematical Bold equivalent.
fn to_unicode_bold(c: char) -> char {
    match c {
        'A'..='Z' => char::from_u32(0x1D400 + (c as u32 - 'A' as u32)).unwrap_or(c),
        'a'..='z' => char::from_u32(0x1D41A + (c as u32 - 'a' as u32)).unwrap_or(c),
        '0'..='9' => char::from_u32(0x1D7CE + (c as u32 - '0' as u32)).unwrap_or(c),
        _ => c,
    }
}

/// Convert a single ASCII char to its Unicode Mathematical Italic equivalent.
fn to_unicode_italic(c: char) -> char {
    match c {
        'A'..='Z' => char::from_u32(0x1D434 + (c as u32 - 'A' as u32)).unwrap_or(c),
        'a'..='z' => char::from_u32(0x1D44E + (c as u32 - 'a' as u32)).unwrap_or(c),
        _ => c,
    }
}

/// Convert a single ASCII char to its Unicode Mathematical Bold Italic equivalent.
fn to_unicode_bold_italic(c: char) -> char {
    match c {
        'A'..='Z' => char::from_u32(0x1D468 + (c as u32 - 'A' as u32)).unwrap_or(c),
        'a'..='z' => char::from_u32(0x1D482 + (c as u32 - 'a' as u32)).unwrap_or(c),
        '0'..='9' => char::from_u32(0x1D7CE + (c as u32 - '0' as u32)).unwrap_or(c),
        _ => c,
    }
}

/// Convert markdown-style formatting to Unicode styled characters for platforms
/// that don't support markdown (e.g. LinkedIn).
/// - `***text***` → Unicode Bold Italic
/// - `**text**` → Unicode Bold
/// - `*text*` → Unicode Italic
fn markdown_to_unicode(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        // Check for *** (bold italic)
        if i + 2 < len && chars[i] == '*' && chars[i + 1] == '*' && chars[i + 2] == '*' {
            if let Some(content) = extract_between(&chars, i + 3, "***") {
                for c in content.chars() {
                    result.push(to_unicode_bold_italic(c));
                }
                i += 3 + content.len() + 3;
                continue;
            }
        }
        // Check for ** (bold)
        if i + 1 < len && chars[i] == '*' && chars[i + 1] == '*' {
            if let Some(content) = extract_between(&chars, i + 2, "**") {
                for c in content.chars() {
                    result.push(to_unicode_bold(c));
                }
                i += 2 + content.len() + 2;
                continue;
            }
        }
        // Check for * (italic) — but not ** which is handled above
        if chars[i] == '*' && (i + 1 >= len || chars[i + 1] != '*') {
            if let Some(content) = extract_between(&chars, i + 1, "*") {
                for c in content.chars() {
                    result.push(to_unicode_italic(c));
                }
                i += 1 + content.len() + 1;
                continue;
            }
        }
        result.push(chars[i]);
        i += 1;
    }

    result
}

/// Extract text between the current position and the next occurrence of the closing delimiter.
fn extract_between(chars: &[char], start: usize, delim: &str) -> Option<String> {
    let delim_chars: Vec<char> = delim.chars().collect();
    let dlen = delim_chars.len();

    for end in start..=(chars.len().saturating_sub(dlen)) {
        if chars[end..end + dlen] == delim_chars[..] {
            let content: String = chars[start..end].iter().collect();
            if !content.is_empty() {
                return Some(content);
            }
            return None;
        }
    }
    None
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

/// Rewrite text using Groq.
/// Style rewrites use Llama 3.3 70B; skill rewrites (custom_prompt) use Qwen3 32B for richer results.
pub fn rewrite_text(text: &str, style: &RewriteStyle, api_key: &str, usage: Option<&Mutex<GroqUsage>>, custom_prompt: Option<&str>) -> Result<String> {
    let (prompt, model) = match custom_prompt {
        Some(p) => (p, "qwen/qwen3-32b"),
        None => (system_prompt(style), "llama-3.3-70b-versatile"),
    };
    let body = serde_json::json!({
        "model": model,
        "messages": [
            { "role": "system", "content": prompt },
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
        .map(|c| strip_think_tags(&c.message.content).trim().to_string())
        .unwrap_or_default();

    if content.is_empty() {
        return Err(anyhow!("Empty response from Groq"));
    }

    // Convert markdown bold/italic to Unicode for skill rewrites (e.g. LinkedIn)
    let final_content = if custom_prompt.is_some() {
        markdown_to_unicode(&content)
    } else {
        content
    };

    Ok(final_content)
}
