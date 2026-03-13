# Local Development Notes

## Recent Changes

### fix: overlay popup appears over full-screen apps via NSPanel swizzle - 2026-02-21
- Branch: `claude/fix-popup-z-index-8DY88`
- PR: https://github.com/genesis1tech/vox2txt/pull/19
- Summary: Swizzle overlay NSWindow to NSPanel at runtime so it renders over macOS full-screen Spaces. Removed conflicting always_on_top, set NonactivatingPanel style mask to prevent focus stealing.
