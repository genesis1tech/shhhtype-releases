# voice2txt - Claude Code Guidelines

## Project Overview

macOS menu bar voice-to-text tool for developers. Captures speech via global hotkey and injects transcribed text into any focused application (IDEs, terminals, browsers).

## Tech Stack

- **Framework:** Tauri v2 (Rust backend) + React 19 + TypeScript + Tailwind CSS v4 + Vite 7
- **STT:** whisper-rs (Rust bindings for whisper.cpp) with Metal GPU acceleration
- **Audio:** cpal crate for microphone capture
- **Text Injection:** arboard (clipboard) + core-graphics (CGEvent Cmd+V simulation)
- **Storage:** rusqlite with bundled SQLite
- **Hotkeys:** tauri-plugin-global-shortcut

## Architecture

```
Menu Bar Tray App (no main window)
├── Global Hotkey (push-to-talk / toggle)
├── Audio Pipeline (Rust): cpal → resample 16kHz → VAD → buffer
├── Transcription (Rust): whisper-rs + Metal GPU → dictionary corrections
├── Text Injection (Rust): clipboard paste or keyboard simulation
├── Overlay Window (React): transparent floating pill with status
└── Settings Window (React): tabbed config panel
```

## Key Directories

- `src-tauri/src/` - Rust backend (modules: audio, transcribe, inject, hotkey, vad, config, db, tray)
- `src/` - React frontend (components, hooks, lib)
- `src-tauri/resources/` - Default dictionary JSON

## Commands

```bash
# Development
npm run tauri dev          # Run in dev mode (frontend + Rust)
npx vite build             # Build frontend only
cargo check --manifest-path src-tauri/Cargo.toml  # Check Rust only
cargo build --manifest-path src-tauri/Cargo.toml  # Build Rust only

# Production
npm run tauri build        # Full production build
```

## Conventions

- Tauri commands in `src-tauri/src/commands.rs`, invoked from `src/lib/commands.ts`
- TypeScript types mirror Rust structs in `src/lib/types.ts`
- Settings persisted as JSON in `~/Library/Application Support/com.g1tech.voice2txt/`
- SQLite database at same location for transcription history
- All audio processing happens in Rust (no JS audio APIs)
- Use `parking_lot::Mutex` and `RwLock` instead of std (no poisoning)
- Tray-only app: `tauri.conf.json` has empty `windows: []`, windows created dynamically

## Implementation Status

- Phase 1 (Skeleton): Complete - tray icon, module structure, compiles
- Phase 2 (Audio Capture): Stubs ready, cpal capture implemented
- Phase 3 (Transcription): Stubs ready, whisper-rs dependency added
- Phase 4 (Text Injection): Stubs ready, CGEvent code written
- Phase 5 (Hotkeys): Stub ready
- Phase 6 (Frontend UI): Component stubs created
- Phase 7 (Persistence): SQLite schema and CRUD implemented
- Phase 8 (Polish): Pending

## Important Notes

- Requires macOS Accessibility permission for text injection
- Requires Microphone permission for audio capture
- whisper-rs needs cmake installed for building whisper-sys
- Metal feature flag enables M-series GPU acceleration
- `macOSPrivateApi: true` in tauri.conf.json enables transparent overlay windows
