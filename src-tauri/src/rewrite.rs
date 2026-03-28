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

/// 𝐁𝐨𝐥𝐝 𝐒𝐞𝐫𝐢𝐟 — Unicode Mathematical Bold (U+1D400)
fn to_unicode_bold(c: char) -> char {
    match c {
        'A'..='Z' => char::from_u32(0x1D400 + (c as u32 - 'A' as u32)).unwrap_or(c),
        'a'..='z' => char::from_u32(0x1D41A + (c as u32 - 'a' as u32)).unwrap_or(c),
        '0'..='9' => char::from_u32(0x1D7CE + (c as u32 - '0' as u32)).unwrap_or(c),
        _ => c,
    }
}

/// 𝑰𝒕𝒂𝒍𝒊𝒄 𝑺𝒆𝒓𝒊𝒇 — Unicode Mathematical Italic (U+1D434)
/// Note: U+1D455 (italic h) is unassigned; correct mapping is U+210E (ℎ PLANCK CONSTANT).
fn to_unicode_italic(c: char) -> char {
    match c {
        'A'..='Z' => char::from_u32(0x1D434 + (c as u32 - 'A' as u32)).unwrap_or(c),
        'h' => '\u{210E}', // PLANCK CONSTANT — the only gap in Mathematical Italic
        'a'..='z' => char::from_u32(0x1D44E + (c as u32 - 'a' as u32)).unwrap_or(c),
        _ => c,
    }
}

/// 𝑩𝒐𝒍𝒅 𝑰𝒕𝒂𝒍𝒊𝒄 𝑺𝒆𝒓𝒊𝒇 — Unicode Mathematical Bold Italic (U+1D468)
fn to_unicode_bold_italic(c: char) -> char {
    match c {
        'A'..='Z' => char::from_u32(0x1D468 + (c as u32 - 'A' as u32)).unwrap_or(c),
        'a'..='z' => char::from_u32(0x1D482 + (c as u32 - 'a' as u32)).unwrap_or(c),
        '0'..='9' => char::from_u32(0x1D7CE + (c as u32 - '0' as u32)).unwrap_or(c),
        _ => c,
    }
}

/// 𝗕𝗼𝗹𝗱 𝗦𝗮𝗻𝘀-𝗦𝗲𝗿𝗶𝗳 — Unicode Mathematical Sans-Serif Bold (U+1D5D4)
#[allow(dead_code)]
fn to_unicode_bold_sans(c: char) -> char {
    match c {
        'A'..='Z' => char::from_u32(0x1D5D4 + (c as u32 - 'A' as u32)).unwrap_or(c),
        'a'..='z' => char::from_u32(0x1D5EE + (c as u32 - 'a' as u32)).unwrap_or(c),
        '0'..='9' => char::from_u32(0x1D7EC + (c as u32 - '0' as u32)).unwrap_or(c),
        _ => c,
    }
}

/// 𝘐𝘵𝘢𝘭𝘪𝘤 𝘚𝘢𝘯𝘴-𝘚𝘦𝘳𝘪𝘧 — Unicode Mathematical Sans-Serif Italic (U+1D608)
#[allow(dead_code)]
fn to_unicode_italic_sans(c: char) -> char {
    match c {
        'A'..='Z' => char::from_u32(0x1D608 + (c as u32 - 'A' as u32)).unwrap_or(c),
        'a'..='z' => char::from_u32(0x1D622 + (c as u32 - 'a' as u32)).unwrap_or(c),
        _ => c,
    }
}

/// Convert markdown-style formatting to Unicode styled characters for platforms
/// that don't support markdown (e.g. LinkedIn).
/// - `***text***` → 𝑩𝒐𝒍𝒅 𝑰𝒕𝒂𝒍𝒊𝒄
/// - `**text**` → 𝐁𝐨𝐥𝐝
/// - `*text*` → 𝐼𝑡𝑎𝑙𝑖𝑐
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
        // Check for * (italic)
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

/// Strip markdown bold/italic markers, returning plain text.
fn strip_markdown(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        // Check for *** (bold italic)
        if i + 2 < len && chars[i] == '*' && chars[i + 1] == '*' && chars[i + 2] == '*' {
            if let Some(content) = extract_between(&chars, i + 3, "***") {
                result.push_str(&content);
                i += 3 + content.len() + 3;
                continue;
            }
        }
        // Check for ** (bold)
        if i + 1 < len && chars[i] == '*' && chars[i + 1] == '*' {
            if let Some(content) = extract_between(&chars, i + 2, "**") {
                result.push_str(&content);
                i += 2 + content.len() + 2;
                continue;
            }
        }
        // Check for * (italic)
        if chars[i] == '*' && (i + 1 >= len || chars[i + 1] != '*') {
            if let Some(content) = extract_between(&chars, i + 1, "*") {
                result.push_str(&content);
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

/// Rewrite text using Groq (Llama 3.3 70B primary, Qwen3 32B fallback on rate limit).
pub fn rewrite_text(text: &str, style: &RewriteStyle, api_key: &str, usage: Option<&Mutex<GroqUsage>>, custom_prompt: Option<&str>, formatting: bool) -> Result<String> {
    let prompt = custom_prompt.unwrap_or_else(|| system_prompt(style));
    let primary_model = "llama-3.3-70b-versatile";
    let fallback_model = "qwen/qwen3-32b";

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let (resp, model_used) = {
        let body = serde_json::json!({
            "model": primary_model,
            "messages": [
                { "role": "system", "content": prompt },
                { "role": "user", "content": text }
            ],
            "temperature": 0.3,
            "max_tokens": 2048,
        });

        let resp = client
            .post(GROQ_CHAT_URL)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()?;

        if resp.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
            // Update usage from the 429 response — headers still carry valid rate limit info
            if let Some(usage) = usage {
                crate::state::update_groq_usage(resp.headers(), usage);
            }
            log::warn!("Groq rate limit hit for {}, falling back to {}", primary_model, fallback_model);
            let fallback_body = serde_json::json!({
                "model": fallback_model,
                "messages": [
                    { "role": "system", "content": prompt },
                    { "role": "user", "content": text }
                ],
                "temperature": 0.3,
                "max_tokens": 2048,
            });

            let fallback_resp = client
                .post(GROQ_CHAT_URL)
                .header("Authorization", format!("Bearer {}", api_key))
                .header("Content-Type", "application/json")
                .json(&fallback_body)
                .send()?;

            (fallback_resp, fallback_model)
        } else {
            (resp, primary_model)
        }
    };

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().unwrap_or_default();
        return Err(anyhow!("Groq API error {} (model: {}): {}", status, model_used, body));
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
    let final_content = if custom_prompt.is_some() && formatting {
        markdown_to_unicode(&content)
    } else if custom_prompt.is_some() {
        // Formatting disabled — strip markdown markers, return plain text
        strip_markdown(&content)
    } else {
        content
    };

    Ok(final_content)
}
