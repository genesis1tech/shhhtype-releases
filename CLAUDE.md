# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

macOS menu bar voice-to-text tool for developers. Captures speech via global hotkey and injects transcribed text into any focused application (IDEs, terminals, browsers). All transcription runs on-device via whisper.cpp with Metal GPU acceleration (or optionally via Groq cloud API).

## Commands

```bash
# Development (launches Vite dev server + Rust backend together)
RUST_LOG=info npm run tauri dev

# Check/build Rust only (faster iteration on backend changes)
cargo check --manifest-path src-tauri/Cargo.toml
cargo build --manifest-path src-tauri/Cargo.toml

# Build frontend only
npx vite build

# Production build (.app bundle + .dmg installer)
npm run tauri build
# Output: src-tauri/target/release/bundle/macos/vox2txt.app

# Code signing setup (one-time, required for macOS permissions to persist across rebuilds)
./scripts/setup-dev-signing.sh
```

No test suite exists yet. There is no linter configured.

## Architecture

Tray-only app: `tauri.conf.json` has `windows: []` — no main window. Windows are created dynamically in `src-tauri/src/windows.rs`.

**Single React entry point, multi-window routing:** `src/App.tsx` reads `getCurrentWindow().label` and renders either `<Overlay />` (label="overlay") or `<Settings />` (default). Both windows load the same `index.html`.

**Recording flow (hotkey-driven, all in Rust):**
1. Global hotkey registered in `lib.rs::register_hotkey()` — supports PushToTalk and Toggle modes
2. `commands::do_start_recording()` spawns an audio capture thread (`audio::capture`) with VAD monitoring (`vad::energy`)
3. VAD auto-stops after configurable silence timeout, or user releases/re-presses hotkey
4. `commands::do_stop_and_transcribe()` resamples audio to 16kHz (`audio::resampler`), runs Whisper inference (`transcribe::engine`) or Groq API (`transcribe::groq`), applies dictionary corrections (`transcribe::dictionary`)
5. Text injected into focused app via clipboard paste (`inject::clipboard`) or keyboard simulation (`inject::keyboard`)

**State management:** `state::AppState` is a single struct with `parking_lot` locks, managed as `Arc<AppState>` by Tauri. Recording state tracked via `AtomicU8` (idle/recording/transcribing).

**Frontend-backend bridge:** Tauri commands defined in `src-tauri/src/commands.rs`, TypeScript wrappers in `src/lib/commands.ts`, shared types in `src/lib/types.ts`. Events emitted via `app.emit()`: `recording-state-changed`, `transcription-complete`, `model-download-progress`.

## Key Conventions

- Use `parking_lot::Mutex` and `RwLock` (not std) — no poisoning
- Tauri commands are the only Rust<->JS interface; all audio processing stays in Rust
- Settings persisted as JSON, history in SQLite, both at `~/Library/Application Support/com.g1tech.vox2txt/`
- Whisper models downloaded to `{data_dir}/models/` with SHA-256 verification
- `macOSPrivateApi: true` enables transparent overlay windows
- Overlay window uses NSWindow level 1001 (above screensaver) to float over everything including full-screen apps
- CSP configured in `tauri.conf.json` — `connect-src` must include any new external API domains

## Platform Requirements

- macOS only (Apple Silicon recommended for Metal GPU)
- Requires Microphone, Accessibility, and Input Monitoring permissions
- whisper-rs build needs cmake (`brew install cmake`)
- Code signing via `scripts/setup-dev-signing.sh` so permissions persist across dev rebuilds
