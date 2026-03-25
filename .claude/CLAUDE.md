# Local Development Notes

## Recent Changes

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

### minor: add overlay position option — top center vs inline at cursor - 2026-03-23
- Branch: `minor/inline-overlay-position`
- PR: https://github.com/genesis1tech/vox2txt/pull/45
- Summary: OverlayPosition enum (TopCenter default, Inline) with cursor-tracking positioning. Overlay centers on mouse cursor above it with edge clamping. TopCenter now repositions to cursor's screen (multi-monitor fix). Feedback settings moved from Audio to General tab.

### minor: add voice-triggered rewrite skills with LinkedIn skill - 2026-03-20
- Branch: `minor/skill-loader`
- PR: https://github.com/genesis1tech/vox2txt/pull/36
- Summary: Extensible skill system — `.md` files with YAML frontmatter define custom rewrite prompts triggered by voice commands (e.g. "/linkedin skill"). Bundled LinkedIn post optimizer skill. Trigger detection normalizes spoken variants, integrated into hotkey and command rewrite paths.

### minor: hotkey settings UX & transcription perf optimizations - 2026-03-19
- Branch: `minor/hotkey-settings-and-transcription-perf`
- PR: https://github.com/genesis1tech/vox2txt/pull/31
- Summary: Dynamic hotkey changes with restart flow, fixed key capture (e.code), skip resampling for Cloud transcription (~3s saved), 16-bit WAV, HTTP client reuse, dictionary caching, keyboard injection speedup, UI polish.
