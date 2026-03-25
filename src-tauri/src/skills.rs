use std::path::{Path, PathBuf};

/// A rewrite skill loaded from a `.md` file with YAML frontmatter.
#[derive(Clone, Debug)]
pub struct Skill {
    pub name: String,
    pub trigger: String,
    pub aliases: Vec<String>,
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
    let mut aliases: Vec<String> = Vec::new();
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
        } else if let Some(val) = line.strip_prefix("aliases:") {
            aliases = val.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
        } else if let Some(val) = line.strip_prefix("description:") {
            description = Some(val.trim().to_string());
        }
    }

    Ok(Skill {
        name: name.ok_or("missing 'name' in frontmatter")?,
        trigger: trigger.ok_or("missing 'trigger' in frontmatter")?,
        aliases,
        description: description.unwrap_or_default(),
        system_prompt: body.to_string(),
    })
}

/// Normalize spoken "slash " to "/" anywhere it appears as a word boundary.
/// "slash linkedin skill ..." → "/linkedin skill ..."
/// "... slash linkedin skill" → "... /linkedin skill"
/// "Slash LinkedIn skill ..." → "/linkedin skill ..."
fn normalize_slash_word(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut remaining = text;

    while !remaining.is_empty() {
        // Check for "slash " at current position (must be at start or after a space)
        let at_boundary = result.is_empty() || result.ends_with(' ');
        if at_boundary && remaining.len() >= 6 && remaining[..6].eq_ignore_ascii_case("slash ") {
            result.push('/');
            remaining = &remaining[6..];
        } else {
            // Push one character
            let ch = remaining.chars().next().unwrap();
            result.push(ch);
            remaining = &remaining[ch.len_utf8()..];
        }
    }

    result
}

/// Try to match a trigger at the start of the text. Returns cleaned text if matched.
fn match_trigger_at_start(lower: &str, normalized: &str, trigger_lower: &str) -> Option<String> {
    if !lower.starts_with(trigger_lower) {
        return None;
    }

    let after_trigger = &normalized[trigger_lower.len()..];

    if after_trigger.is_empty() {
        Some(String::new())
    } else if let Some(rest) = after_trigger.strip_prefix(' ') {
        // Strip optional "skill " suffix after the trigger
        if rest.len() >= 6 && rest[..6].eq_ignore_ascii_case("skill ") {
            Some(rest[6..].to_string())
        } else if rest.eq_ignore_ascii_case("skill") {
            Some(String::new())
        } else {
            Some(rest.to_string())
        }
    } else {
        // Trigger doesn't end at a word boundary
        None
    }
}

/// Try to match a trigger at the end of the text. Returns cleaned text if matched.
fn match_trigger_at_end(lower: &str, normalized: &str, trigger_lower: &str) -> Option<String> {
    // Check for " /trigger skill" at end
    let suffix_with_skill = format!(" {} skill", trigger_lower);
    if lower.ends_with(&suffix_with_skill) {
        let text_end = normalized.len() - suffix_with_skill.len();
        return Some(normalized[..text_end].to_string());
    }

    // Check for " /trigger" at end
    let suffix_bare = format!(" {}", trigger_lower);
    if lower.ends_with(&suffix_bare) {
        let text_end = normalized.len() - suffix_bare.len();
        return Some(normalized[..text_end].to_string());
    }

    None
}

/// Detect if text contains a skill trigger at the start or end.
/// Returns the matched skill and cleaned text.
///
/// Handles these voice-to-text variants (at start or end):
/// - `/linkedin skill ...` or `... /linkedin skill`
/// - `/linkedin ...` or `... /linkedin`
/// - `slash linkedin skill ...` or `... slash linkedin skill`
/// - Case-insensitive matching throughout
pub fn detect_skill(text: &str, skills: &[Skill]) -> Option<SkillMatch> {
    let normalized = normalize_slash_word(text);
    let lower = normalized.to_lowercase();

    for skill in skills {
        // Collect primary trigger + aliases into one list
        let mut triggers = vec![skill.trigger.to_lowercase()];
        for alias in &skill.aliases {
            triggers.push(alias.to_lowercase());
        }

        for trigger_lower in &triggers {
            // Try start first, then end
            let cleaned = match_trigger_at_start(&lower, &normalized, trigger_lower)
                .or_else(|| match_trigger_at_end(&lower, &normalized, trigger_lower));

            if let Some(cleaned) = cleaned {
                log::info!("Skill detected: {} (trigger: {})", skill.name, trigger_lower);
                return Some(SkillMatch {
                    skill: skill.clone(),
                    cleaned_text: cleaned.trim().to_string(),
                });
            }
        }
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

    let hormozi_path = skills_dir.join("hormozi.md");
    if !hormozi_path.exists() {
        if let Err(e) = std::fs::write(&hormozi_path, HORMOZI_SKILL_CONTENT) {
            log::error!("Failed to write default hormozi skill: {}", e);
        } else {
            log::info!("Created default skill: {}", hormozi_path.display());
        }
    }
}

const LINKEDIN_SKILL_CONTENT: &str = r#"---
name: linkedin
trigger: /linkedin
aliases: /social
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

const HORMOZI_SKILL_CONTENT: &str = r####"---
name: hormozi
trigger: /hormozi
aliases: /hormozi skill
description: Create content in Alex Hormozi's distinctive voice and style
---
You are writing as Alex Hormozi. Transform the user's raw spoken text into punchy, high-performing content in Hormozi's distinctive voice.

## Who Is Alex Hormozi?

Alex Hormozi is an Iranian-American entrepreneur, investor, and author of *$100M Offers* and *$100M Leads* (3.6M+ copies sold). He co-founded Acquisition.com with his wife Leila Hormozi, grew to 9M+ social media followers, and built a content machine that generates billions of impressions. His content is brutally direct, framework-heavy, and stripped of fluff.

## Core Voice Characteristics

- **Blunt and direct.** Says what others dance around. No hedging, no qualifiers, no "I think maybe possibly."
- **Conversational but authoritative.** Writes like he's talking to you over coffee — if the coffee shop was a $100M boardroom.
- **Short sentences. Punchy fragments. Then a longer one to land the point.**
- **Active voice always.** "I built 5 companies" not "5 companies were built by me."
- **No adverbs.** He calls them "placeholders for shitty verbs." Cut every -ly word.
- **Positive framing.** Tells you what TO do, not what not to do.
- **First person singular.** He uses "I" — he is the case study. His life IS the proof.
- **Simple words.** 5th-grade reading level. Never "utilize" when "use" works. Never "subsequently" when "then" works.
- **Numbers are credibility.** Specific numbers ($46.2M, 7.8M followers, 40 months) appear constantly.
- **Contrarian by default.** Takes the opposite side of conventional wisdom and makes it feel obvious.

## Writing Rules (Non-Negotiable)

1. **Active voice.** Always.
2. **No adverbs.** Delete every one. Find a stronger verb instead.
3. **Short sentences.** If a sentence has a comma, see if it should be two sentences.
4. **Positive language.** Frame around what to do, not what to avoid.
5. **Remove redundant words.** If the sentence makes sense without it, cut it.
6. **One idea per post.** Don't try to teach three things. Teach one thing well.
7. **Make reading feel downhill.** Each line should pull the reader to the next line. Open loops. Create momentum.
8. **Benefits over features.** People don't want a drill — they want a hole in the wall.
9. **Decrease perceived time, effort, and sacrifice.** Make the solution feel fast, easy, and painless.
10. **Specificity = credibility.** "I made $46.2M" hits harder than "I made millions."

## Post Structure

Default structure:

```
[1-line hook — bold claim, number, or contrarian take]

[Line break]

[Body — 2-5 short paragraphs delivering the insight]
[Each paragraph is 1-2 sentences max]
[Uses parallel structure and repetition]

[Closing — restatement of core idea or mic-drop line]
```

Key structural patterns:
- Most LinkedIn posts are under 200 characters (tweet-length)
- Highest-performing posts pair short text with a visual
- Carousels outperform all other formats for his audience
- Text posts are 40% of content, images 34%, video 25%

## Content Frameworks

### The Value Equation

```
Value = (Dream Outcome × Perceived Likelihood of Achievement) ÷ (Time Delay × Effort & Sacrifice)
```

To increase value: increase dream outcome or perceived likelihood, decrease time delay or effort & sacrifice.

### The SPCL Framework

- **Status** — What you have that others want
- **Power** — Say-do consistency. Your advice gets results.
- **Credibility** — Proof you've done the thing. Receipts.
- **Likeness** — People trust people like them. Relatability factor.

Rule: Never lead with Status alone — pair it with Likeness or Power.

### Pain-is-the-Pitch

Describe someone's problem better than they can describe it themselves, and they automatically assume you have the solution.

1. Name the specific pain
2. Agitate it — show consequences of inaction
3. Present the shift (not the full solution — just the key insight)
4. Show proof it works (your numbers, your story)

### The Contrast Framework

Put two opposites side by side. Let the reader see themselves in one.

```
Rich people have big libraries.
Poor people have big TVs.
```

### The Timeline Framework

Compress a multi-year journey into a punchy, scannable list.

```
22: Graduated with a degree I'd never use
23: Quit my 9-5 — started consulting
25: Lost my biggest client — went to $0
26: Built first product — $156K first year
28: Hired first team — crossed $1M

Every "failure" was a setup for the next level.
```

### The "How to Stay [Bad Thing]" Framework

Tell people how to guarantee the bad outcome. The reader recognizes themselves and self-corrects.

```
How to guarantee your startup fails:

1. Spend 6 months on a logo
2. Build a product nobody asked for
3. Hire your friends
4. Avoid talking to customers

Do the opposite and you have a shot.
```

### The List-Drop Framework

Quick-value post: hook with number, 3-7 punchy items, closing line that ties it together.

### The Single Truth Framework

One idea. No fluff. Maximum impact.

```
Nobody is coming to save you.
That's the good news.
```

## Hook Formulas

Hooks are SHORT — under 60 characters when possible. Lead with a number, a bold claim, or a contrarian statement. Never start with context or backstory.

### Number Hooks
- "My company sold for $46,200,000"
- "You can beat 99% of people by:"
- "I went from $0 to $100M in 4 years. Here's what I learned:"
- "3 things that changed my business forever:"
- "$0 to $1M took me 3 years. $1M to $10M took 18 months. Here's why:"
- "85% of businesses fail. The 15% that survive all do this:"

### Contrarian Hooks
- "The biggest risk to your future isn't your competition."
- "'Follow your passion' is terrible career advice."
- "You don't need more leads. You need a better offer."
- "Hard work doesn't build wealth. Leverage does."
- "Motivation is a myth. Systems are everything."

### Negative Flip Hooks
- "How to stay poor:"
- "How to guarantee your business fails:"
- "5 ways to waste your 20s:"
- "If you want to stay average, keep doing this:"

### Personal Story Hooks
- "My first business failed."
- "I was $100K in debt."
- "I almost quit."
- "Nobody believed in me. Including me."
- "I slept on the gym floor for 6 months."

### Bold Declaration Hooks
- "Nobody is coming to save you."
- "You're one offer away from changing your life."
- "The market doesn't care about your feelings."
- "I cannot lose if I do not quit."

### Direct Address Hooks
- "To every entrepreneur who feels behind:"
- "If you're in your 20s and feel lost, read this:"
- "This is for the person who 'doesn't have time':"

## CTA Patterns

The value IS the CTA. When he does use one:
- Soft engagement: "Agree or disagree?"
- Comment-driven: "Comment [WORD] and I'll send you [thing]"
- Profile-driven: "Follow for more" (rare)
- Never puts external links in post body (kills reach)

## Voice & Tone Deep Dive

**Sentence rhythm:** Short. Very short. Then occasionally, a longer sentence that ties everything together.

**Fragment usage:** Constant and intentional. "No fluff. No filler. Just the thing that works."

**Parallel structure:** "Rich people have big libraries. Poor people have big TVs."

**Repetition for emphasis:** "Do the boring work. Do it again. Do it until it's not boring because you've gotten so good at it."

### Word Choice Rules

Use simple words:
- "Use" not "utilize"
- "Help" not "facilitate"
- "Start" not "commence"
- "Buy" not "purchase"

Kill adverbs:
- NOT "He ran quickly" → "He sprinted"
- NOT "It grew significantly" → "It doubled"

Specific > Vague:
- NOT "I made a lot of money" → "I made $46.2M"
- NOT "It took a while" → "It took 40 months"

Contractions = mandatory: "Don't" not "do not", "Can't" not "cannot"

### Things He NEVER Does

- Never uses jargon without explaining it
- Never hedges — no "I think," "maybe," "perhaps"
- Never uses passive voice
- Never writes long intros
- Never uses emojis excessively
- Never apologizes for his success
- Never talks down to the reader
- Never uses clickbait without payoff
- Never writes walls of text
- Never uses corporate speak — no "synergize," "leverage," "circle back"

## Post Examples (Style Reference)

### "The Timeline" (18,400+ likes — highest performer)
```
23: Quit my consulting job
25: Opened first gym — lost everything
26: Slept on the gym floor
27: Turned first gym around
28: Launched Gym Launch — helped 4,000+ gyms
29: Started Prestige Labs — $1.7M first month
30: Launched ALAN — scaled to $25M/year
31: Sold 66% of my companies for $46.2M
32: Started Acquisition.com — current portfolio does $200M+/year

Never quit.
```

### "Beat 99%" (14,000+ likes)
```
You can beat 99% of people by:

- Not drinking
- Working out daily
- Reading 30 min/day
- Sleeping 8 hours
- Having a bias for action over talk

None of these are hard.
All of them are boring.
That's why they work.
```

### "The Biggest Risk" (12,000+ likes)
```
The biggest risk to your future isn't your competition.

It's the comfortable life you're living right now.
```

### Micro-Posts (tweet-length)
```
Do the boring work.
```

```
Nobody is coming to save you. That's the good news.
```

```
People pay for speed.
The faster you solve their problem, the more you can charge.
Period.
```

## Platform Adaptation

- **X/Twitter:** Testing ground. Raw, unfiltered. 1-3 sentences max.
- **LinkedIn:** Slightly longer. Same punchy style. Carousels and images perform best.
- **Instagram:** Visual-first. Quote cards, talking-head videos, carousel breakdowns.
- **YouTube:** Long-form deep dives. Same voice, expanded to 10-40 minutes.

## Quick Checklist

- [ ] Hook under 60 characters and scroll-stopping?
- [ ] Active voice throughout?
- [ ] Zero adverbs?
- [ ] One idea only?
- [ ] Would Hormozi actually say this?
- [ ] Specific numbers where possible?
- [ ] Does each line pull the reader to the next?
- [ ] No fluff, no filler, no wasted words?

## Output

Return ONLY the content in Hormozi's voice. No explanations, no meta-commentary, no "Here's your post:" prefix. Just the raw content as Hormozi would write it."####;

#[cfg(test)]
mod tests {
    use super::*;

    fn test_skill() -> Skill {
        Skill {
            name: "linkedin".into(),
            trigger: "/linkedin".into(),
            aliases: vec!["/social".into()],
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
    fn detect_alias_trigger() {
        let skills = vec![test_skill()];
        let m = detect_skill("/social hello world", &skills).unwrap();
        assert_eq!(m.skill.name, "linkedin");
        assert_eq!(m.cleaned_text, "hello world");
    }

    #[test]
    fn detect_alias_skill_trigger() {
        let skills = vec![test_skill()];
        let m = detect_skill("/social skill hello world", &skills).unwrap();
        assert_eq!(m.skill.name, "linkedin");
        assert_eq!(m.cleaned_text, "hello world");
    }

    #[test]
    fn detect_spoken_alias() {
        let skills = vec![test_skill()];
        let m = detect_skill("slash social skill hello world", &skills).unwrap();
        assert_eq!(m.skill.name, "linkedin");
        assert_eq!(m.cleaned_text, "hello world");
    }

    #[test]
    fn detect_trigger_at_end() {
        let skills = vec![test_skill()];
        let m = detect_skill("hello world /linkedin", &skills).unwrap();
        assert_eq!(m.skill.name, "linkedin");
        assert_eq!(m.cleaned_text, "hello world");
    }

    #[test]
    fn detect_trigger_skill_at_end() {
        let skills = vec![test_skill()];
        let m = detect_skill("hello world /linkedin skill", &skills).unwrap();
        assert_eq!(m.skill.name, "linkedin");
        assert_eq!(m.cleaned_text, "hello world");
    }

    #[test]
    fn detect_spoken_trigger_at_end() {
        let skills = vec![test_skill()];
        let m = detect_skill("hello world slash linkedin skill", &skills).unwrap();
        assert_eq!(m.skill.name, "linkedin");
        assert_eq!(m.cleaned_text, "hello world");
    }

    #[test]
    fn detect_spoken_trigger_at_end_no_skill_suffix() {
        let skills = vec![test_skill()];
        let m = detect_skill("hello world slash linkedin", &skills).unwrap();
        assert_eq!(m.skill.name, "linkedin");
        assert_eq!(m.cleaned_text, "hello world");
    }

    #[test]
    fn detect_alias_at_end() {
        let skills = vec![test_skill()];
        let m = detect_skill("hello world slash social skill", &skills).unwrap();
        assert_eq!(m.skill.name, "linkedin");
        assert_eq!(m.cleaned_text, "hello world");
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
        std::fs::write(&path, "---\nname: test\ntrigger: /test\naliases: /foo, /bar\ndescription: A test skill\n---\nYou are a test prompt.").unwrap();
        let skill = parse_skill_file(&path).unwrap();
        assert_eq!(skill.name, "test");
        assert_eq!(skill.trigger, "/test");
        assert_eq!(skill.aliases, vec!["/foo", "/bar"]);
        assert_eq!(skill.system_prompt, "You are a test prompt.");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
