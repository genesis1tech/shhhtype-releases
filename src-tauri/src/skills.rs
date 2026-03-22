use std::path::{Path, PathBuf};

/// A rewrite skill loaded from a `.md` file with YAML frontmatter.
#[derive(Clone, Debug)]
pub struct Skill {
    pub name: String,
    pub trigger: String,
    pub description: String,
    pub system_prompt: String,
}

/// Result of detecting a skill trigger in transcription text.
pub struct SkillMatch {
    pub skill: Skill,
    pub cleaned_text: String,
}

/// Load all skills from `.md` files in `{data_dir}/skills/`.
pub fn load_skills(data_dir: &Path) -> Vec<Skill> {
    let skills_dir = data_dir.join("skills");
    if !skills_dir.exists() {
        return Vec::new();
    }

    let mut skills = Vec::new();
    let entries = match std::fs::read_dir(&skills_dir) {
        Ok(e) => e,
        Err(e) => {
            log::warn!("Failed to read skills directory: {}", e);
            return Vec::new();
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        match parse_skill_file(&path) {
            Ok(skill) => {
                log::info!("Loaded skill: {} (trigger: {})", skill.name, skill.trigger);
                skills.push(skill);
            }
            Err(e) => {
                log::warn!("Failed to parse skill file {}: {}", path.display(), e);
            }
        }
    }

    skills
}

/// Parse a single skill `.md` file with YAML frontmatter.
fn parse_skill_file(path: &PathBuf) -> Result<Skill, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("read error: {}", e))?;

    // Expect YAML frontmatter between --- delimiters
    if !content.starts_with("---") {
        return Err("missing YAML frontmatter".into());
    }

    let after_first = &content[3..];
    let end_idx = after_first.find("---")
        .ok_or("missing closing --- in frontmatter")?;

    let frontmatter = &after_first[..end_idx];
    let body = after_first[end_idx + 3..].trim();

    if body.is_empty() {
        return Err("empty system prompt".into());
    }

    // Simple YAML parsing for known keys
    let mut name = None;
    let mut trigger = None;
    let mut description = None;

    for line in frontmatter.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(val) = line.strip_prefix("name:") {
            name = Some(val.trim().to_string());
        } else if let Some(val) = line.strip_prefix("trigger:") {
            trigger = Some(val.trim().to_string());
        } else if let Some(val) = line.strip_prefix("description:") {
            description = Some(val.trim().to_string());
        }
    }

    Ok(Skill {
        name: name.ok_or("missing 'name' in frontmatter")?,
        trigger: trigger.ok_or("missing 'trigger' in frontmatter")?,
        description: description.unwrap_or_default(),
        system_prompt: body.to_string(),
    })
}

/// Normalize common voice transcription variants of slash commands.
/// "slash linkedin skill ..." → "/linkedin skill ..."
/// "Slash LinkedIn skill ..." → "/linkedin skill ..."
fn normalize_trigger_prefix(text: &str) -> String {
    // Case-insensitive check for leading "slash "
    if text.len() >= 6 && text[..6].eq_ignore_ascii_case("slash ") {
        format!("/{}", &text[6..])
    } else {
        text.to_string()
    }
}

/// Detect if text starts with any skill trigger. Returns the matched skill and cleaned text.
///
/// Handles these voice-to-text variants:
/// - `/linkedin skill ...` → matches trigger `/linkedin`
/// - `/linkedin ...` → matches
/// - `slash linkedin skill ...` → normalized to `/linkedin skill`, matches
/// - `Slash LinkedIn skill ...` → case-insensitive, matches
pub fn detect_skill(text: &str, skills: &[Skill]) -> Option<SkillMatch> {
    let normalized = normalize_trigger_prefix(text);
    let lower = normalized.to_lowercase();

    for skill in skills {
        let trigger_lower = skill.trigger.to_lowercase();

        // Check if text starts with the trigger
        if !lower.starts_with(&trigger_lower) {
            continue;
        }

        let after_trigger = &normalized[trigger_lower.len()..];

        // After the trigger, expect end-of-string, space, or " skill "
        let cleaned = if after_trigger.is_empty() {
            String::new()
        } else if let Some(rest) = after_trigger.strip_prefix(' ') {
            // Strip optional "skill " prefix after the trigger
            if rest.len() >= 6 && rest[..6].eq_ignore_ascii_case("skill ") {
                rest[6..].to_string()
            } else if rest.eq_ignore_ascii_case("skill") {
                // Just "/linkedin skill" with nothing after
                String::new()
            } else {
                rest.to_string()
            }
        } else {
            // Trigger doesn't end at a word boundary
            continue;
        };

        log::info!("Skill detected: {} (trigger: {})", skill.name, skill.trigger);
        return Some(SkillMatch {
            skill: skill.clone(),
            cleaned_text: cleaned.trim().to_string(),
        });
    }

    None
}

/// Ensure the default bundled skills exist in `{data_dir}/skills/`.
pub fn ensure_default_skills(data_dir: &Path) {
    let skills_dir = data_dir.join("skills");
    if let Err(e) = std::fs::create_dir_all(&skills_dir) {
        log::error!("Failed to create skills directory: {}", e);
        return;
    }

    let linkedin_path = skills_dir.join("linkedin.md");
    if !linkedin_path.exists() {
        if let Err(e) = std::fs::write(&linkedin_path, LINKEDIN_SKILL_CONTENT) {
            log::error!("Failed to write default linkedin skill: {}", e);
        } else {
            log::info!("Created default skill: {}", linkedin_path.display());
        }
    }

    let grant_path = skills_dir.join("grant.md");
    if !grant_path.exists() {
        if let Err(e) = std::fs::write(&grant_path, GRANT_WRITER_SKILL_CONTENT) {
            log::error!("Failed to write default grant skill: {}", e);
        } else {
            log::info!("Created default skill: {}", grant_path.display());
        }
    }
}

const LINKEDIN_SKILL_CONTENT: &str = r#"---
name: linkedin
trigger: /linkedin
description: Rewrite as a high-performing LinkedIn post
---
You are a LinkedIn Post Optimizer. Transform the user's raw spoken text into a high-performing LinkedIn post.

## Core Rules

1. **Preserve the user's message** — do not invent facts, claims, or experiences
2. **Write in first person** — match the user's voice and perspective
3. **No emojis in the first line** — the hook must work with words alone
4. **Keep it under 1300 characters** (excluding hashtags) — optimal for mobile feed visibility
5. **One idea per post** — remove tangents, keep it focused

## Post Structure (HVCTA Framework)

1. **Hook** (line 1): Pattern-interrupt opening that stops the scroll. Use one of these patterns:
   - Contrarian: "Most people think X. They're wrong."
   - Confession: "I almost [failed/quit/gave up] on X."
   - Curiosity gap: "The one thing about X nobody talks about:"
   - Bold claim: "X changed everything about how I Y."
   - Question: "Why does everyone ignore X?"

2. **Value** (lines 2-8): The insight, story, or lesson. Use short paragraphs (1-2 sentences each). Add line breaks between paragraphs for readability.

3. **CTA** (last line): Soft engagement prompt. Examples:
   - "What's your take?"
   - "Have you experienced this?"
   - "Drop your best tip below."
   - "Agree or disagree?"

## Formatting & Spacing Rules (CRITICAL — Matt Gray style)

The post MUST be easy to scan. Every line should breathe. Never write dense paragraphs.

- **Bold the entire first sentence (the hook)** using **text** markdown syntax
- First line: hook only, no emoji, end with period or colon
- **One to two sentences per line, MAX** — then a blank line
- **Blank line between EVERY line or thought** — no exceptions
- Never write more than 2 sentences without a blank line break
- The reader should be able to scroll down the post smoothly, one thought at a time
- Use "→" or "•" for lists instead of numbers (feels less formal)
- Last line: CTA as its own paragraph
- The post should look like a vertical flow of short, punchy lines — NOT blocks of text

## Text Emphasis (CRITICAL for engagement)

LinkedIn supports **bold**, *italic*, and ***bold italic*** markdown. Use them strategically:
- **Bold** key phrases, stats, results, or takeaways that a skimmer should catch (e.g. "**reduced costs by 40%**", "**built it in 2 days**")
- *Italic* for conversational asides, reflections, or emotional beats (e.g. "*And honestly? I almost didn't.*")
- ***Bold italic*** sparingly for the single most impactful phrase in the post
- Do NOT over-format — aim for 2-4 bold phrases and 1-2 italic phrases per post
- Never bold or italicize entire paragraphs — only words or short phrases

## Tone

- Conversational but credible
- Confident without being arrogant
- Specific over vague ("**increased revenue 34%**" vs "grew the business")
- Active voice always

## Hashtags

- Add exactly 4-6 hashtags at the very end of the post, after a blank line
- Hashtags must be relevant to the specific topic of the post
- Mix broad reach tags (e.g. #Leadership, #AI) with niche tags (e.g. #VoiceTech, #SaaS)
- Use CamelCase for readability (e.g. #MachineLearning not #machinelearning)

## Output

Return ONLY the LinkedIn post text followed by hashtags. No explanations, no meta-commentary, no "Here's your post:" prefix. Use **bold**, *italic*, and ***bold italic*** markdown directly in the output text for emphasis."#;

const GRANT_WRITER_SKILL_CONTENT: &str = r###"---
name: grant
trigger: /grant
description: Transform spoken ideas into polished grant proposal sections
---
You are a Grant Writer Skill. Transform the user's raw spoken text into polished, fundable grant proposal content.

The user will speak their ideas about a project, program, or initiative that needs funding. Your job is to restructure their spoken thoughts into clear, compelling grant proposal language that reads like it was written by an experienced grant professional — not generated by AI.

## Core Philosophy

1. **Preserve the user's vision** — never invent programs, statistics, or claims they didn't mention
2. **Every grant is about money** — always frame the proposal around a specific funding request with clear budget justification
3. **Human voice first** — write the way a passionate, knowledgeable person would speak to a funder across a table, not the way a machine generates text
4. **Show, don't tell** — use concrete details, real numbers, and specific examples instead of vague aspirational language
5. **Funders are people** — they read dozens of proposals; yours needs to connect emotionally AND logically

## Writing Voice & Tone

This is CRITICAL. The output must sound like a real human wrote it — someone who deeply cares about this work.

### DO:
- Use first-person plural: "We have spent three years building..." / "Our team includes..."
- Use contractions naturally: "we've seen," "it's clear," "don't"
- Vary sentence length — mix short punchy sentences with longer explanatory ones
- Include sensory and concrete language: "a mother working two jobs who can't afford childcare" not "underserved populations"
- Use active voice: "We will train 50 educators" not "50 educators will be trained"
- Be direct and confident without being arrogant
- Let data speak with context: "In our county, 1 in 4 children go to bed hungry — that's 3,200 kids"

### DO NOT:
- Use AI-sounding phrases: "furthermore," "it is important to note," "in conclusion," "leverage," "utilize," "facilitate," "stakeholders" (unless quoting a funder)
- Use overly formal or academic language that feels stiff
- Start consecutive sentences with the same word
- Write perfectly uniform sentence lengths (a dead giveaway of AI)
- Use passive voice unless absolutely necessary
- Include filler qualifiers: "very," "really," "extremely," "significantly"
- Use buzzwords without substance: "innovative," "transformative," "cutting-edge," "synergy"

## Your Job

Take the user's raw spoken words and rewrite them the way an experienced, passionate grant writer would write them. That's it.

Don't classify. Don't generate sections. Don't add boilerplate. Don't produce content the user didn't talk about.

Understand what the user is trying to say — their intent, their meaning, their message — and express it in polished, fundable, grant-quality language that sounds completely human.

If they're describing a need, make the need compelling. If they're explaining costs, make the budget clear and justified. If they're talking about their vision, make it resonate. Whatever they say, rewrite it through the lens of someone who writes winning grant proposals for a living.

## Rewriting Principles

- Preserve the user's meaning, facts, and intent completely
- Tighten rambling into clear, purposeful prose — cut filler, keep substance
- Structure their thoughts logically even if they spoke them out of order
- If they mention numbers or data, put them in context so they land with impact
- If they mention people or stories, keep them — human details make grants fundable
- If they mention dollar amounts, tie them clearly to what the money does
- Short paragraphs — 2-4 sentences max, then a blank line
- Bold key figures, dollar amounts, and outcomes using **text** markdown
- Use placeholder brackets [like this] only when a critical detail is clearly missing
- If something important is missing, add a brief "Notes:" line at the end

## Output

Return ONLY the rewritten text. No section headers unless the user was clearly writing a specific section. No meta-commentary. No "Here's your rewrite." Jump straight into the polished content."###;

#[cfg(test)]
mod tests {
    use super::*;

    fn test_skill() -> Skill {
        Skill {
            name: "linkedin".into(),
            trigger: "/linkedin".into(),
            description: "test".into(),
            system_prompt: "test prompt".into(),
        }
    }

    #[test]
    fn detect_slash_trigger() {
        let skills = vec![test_skill()];
        let m = detect_skill("/linkedin hello world", &skills).unwrap();
        assert_eq!(m.skill.name, "linkedin");
        assert_eq!(m.cleaned_text, "hello world");
    }

    #[test]
    fn detect_slash_skill_trigger() {
        let skills = vec![test_skill()];
        let m = detect_skill("/linkedin skill hello world", &skills).unwrap();
        assert_eq!(m.cleaned_text, "hello world");
    }

    #[test]
    fn detect_spoken_slash() {
        let skills = vec![test_skill()];
        let m = detect_skill("slash linkedin skill hello world", &skills).unwrap();
        assert_eq!(m.cleaned_text, "hello world");
    }

    #[test]
    fn detect_case_insensitive() {
        let skills = vec![test_skill()];
        let m = detect_skill("Slash LinkedIn Skill Hello World", &skills).unwrap();
        assert_eq!(m.cleaned_text, "Hello World");
    }

    #[test]
    fn no_match_without_trigger() {
        let skills = vec![test_skill()];
        assert!(detect_skill("hello world", &skills).is_none());
    }

    #[test]
    fn no_match_partial_trigger() {
        let skills = vec![test_skill()];
        assert!(detect_skill("/linkedinprofile hello", &skills).is_none());
    }

    #[test]
    fn parse_frontmatter() {
        let dir = std::env::temp_dir().join("shhhtype_test_skills");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("test.md");
        std::fs::write(&path, "---\nname: test\ntrigger: /test\ndescription: A test skill\n---\nYou are a test prompt.").unwrap();
        let skill = parse_skill_file(&path).unwrap();
        assert_eq!(skill.name, "test");
        assert_eq!(skill.trigger, "/test");
        assert_eq!(skill.system_prompt, "You are a test prompt.");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
