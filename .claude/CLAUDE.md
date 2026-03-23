# Local Development Notes

## Recent Changes

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

### minor: rename app from vox2txt to ShhhType with new logo - 2026-03-17
- Branch: `minor/rename-to-shhhtype`
- PR: https://github.com/genesis1tech/vox2txt/pull/28
- Summary: Full rebrand — new "shh" finger gesture logo with two-tone amber/cyan scheme, renamed all identifiers (crate, bundle ID, data paths, DB), updated all user-facing strings across Rust + React, regenerated all icon assets.

### fix: overlay popup appears over full-screen apps via NSPanel swizzle - 2026-02-21
- Branch: `claude/fix-popup-z-index-8DY88`
- PR: https://github.com/genesis1tech/vox2txt/pull/19
- Summary: Swizzle overlay NSWindow to NSPanel at runtime so it renders over macOS full-screen Spaces. Removed conflicting always_on_top, set NonactivatingPanel style mask to prevent focus stealing.