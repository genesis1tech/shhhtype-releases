# Local Development Notes

## Recent Changes

### minor: rename app from vox2txt to ShhhType with new logo - 2026-03-17
- Branch: `minor/rename-to-shhhtype`
- PR: https://github.com/genesis1tech/vox2txt/pull/28
- Summary: Full rebrand — new "shh" finger gesture logo with two-tone amber/cyan scheme, renamed all identifiers (crate, bundle ID, data paths, DB), updated all user-facing strings across Rust + React, regenerated all icon assets.

### fix: overlay popup appears over full-screen apps via NSPanel swizzle - 2026-02-21
- Branch: `claude/fix-popup-z-index-8DY88`
- PR: https://github.com/genesis1tech/vox2txt/pull/19
- Summary: Swizzle overlay NSWindow to NSPanel at runtime so it renders over macOS full-screen Spaces. Removed conflicting always_on_top, set NonactivatingPanel style mask to prevent focus stealing.
