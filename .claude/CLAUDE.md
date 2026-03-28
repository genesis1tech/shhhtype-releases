# Local Development Notes

## Recent Changes

### minor: align website messaging with GTM playbook - 2026-03-28
- Branch: `minor/5-shhhtype-landing-page`
- PR: https://github.com/genesis1tech/vox2txt/pull/66
- Summary: Rewrote all website copy to match the messaging playbook's three pillars (Speed, Authenticity, Consistency). Hero headline "Speak it. Publish it." with 30min→60sec framing. Features reframed as outcomes. New "Who It's For" section for 3 ICP tiers. Pro section adds competitor price comparison. All meta/SEO updated.

### minor: add LinkedIn DM and Connect skills, Skills settings tab - 2026-03-25
- Branch: `minor/linkedin-dm-connect-skills-and-settings-tab`
- PR: https://github.com/genesis1tech/vox2txt/pull/54
- Summary: New /dm skill (6 DM types with anti-spam guardrails, 300-char target), /connect skill (200-char connection note), Skills tab in Settings showing all loaded skills with triggers and spoken equivalents. Removed grant skill (auto-cleanup on startup). Switched rewrite model to Qwen3 32B.

### minor: add skill aliases, end-of-text triggers, and Hormozi skill - 2026-03-25
- Branch: `minor/52-skill-aliases-and-hormozi`
- PR: https://github.com/genesis1tech/vox2txt/pull/52
- Summary: Skills now support aliases (LinkedIn also triggers on /social). Trigger detection works at both start and end of transcription. Bundled Hormozi content skill with voice guidelines, frameworks, hooks, and post examples. Skills staging folder at src-tauri/skills/.

### minor: add 7-day trial, license security, and keychain-based protection - 2026-03-25
- Branch: `minor/licensing-trial-security`
- PR: https://github.com/genesis1tech/vox2txt/pull/49
- Summary: 7-day trial with full feature access, trial start in macOS Keychain (anti-tamper), LemonSqueezy online validation every 24h, license.json requires Keychain key, all features blocked on expiry, License tab redesign with countdown UI, dynamic app version in About tab.

### docs: add Terms and Conditions and Privacy Policy - 2026-03-25
- Branch: `docs/add-terms-and-privacy`
- PR: https://github.com/genesis1tech/vox2txt/pull/48
- Summary: Terms (17 sections) and Privacy Policy (11 sections) for Genesis 1 Technologies, LLC. Legal links in About tab open shhhtype.com/terms and shhhtype.com/privacy. Copyright notice added.
