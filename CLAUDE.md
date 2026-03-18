# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

ShhhType — macOS menu bar voice-to-text tool for developers. Global hotkey captures speech and injects transcribed text into any focused application. Tauri v2 (Rust backend) + React 19 + TypeScript + Tailwind CSS v4 + Vite 7.

## Commands

```bash
# Development (hot-reload frontend + Rust backend)
RUST_LOG=info npm run tauri dev

# Fast Rust-only iteration
cargo check --manifest-path src-tauri/Cargo.toml
cargo build --manifest-path src-tauri/Cargo.toml

# Frontend-only build
npx vite build

# Full production build (outputs .app + .dmg)
npm run tauri build
```

**Prerequisites:** Rust (rustup), Node 18+, Xcode CLI tools, cmake (`brew install cmake` — required for whisper-sys).

No test suite exists. Verify changes manually via `npm run tauri dev`.

## Architecture

### Data Flow: Hotkey → Text Injection

1. Global hotkey registered in `lib.rs` → triggers `do_start_recording()`
2. cpal stream captures f32 PCM → fills `Arc<Mutex<Vec<f32>>>` in `audio/capture.rs`
3. VAD monitor (`vad/energy.rs`) emits audio-level events to Overlay every 50ms
4. On stop → `do_stop_and_transcribe()` in `commands.rs`:
   - Zero-copy buffer swap (takes entire Vec, leaves empty Vec behind)
   - Resample to 16kHz via rubato SincFixedIn (`audio/resampler.rs`)
   - Route to local Whisper (`transcribe/engine.rs`) or Groq cloud (`transcribe/groq.rs`)
   - Apply dictionary corrections (`transcribe/dictionary.rs`)
   - Save to SQLite history
   - Inject text via clipboard+Cmd+V (`inject/clipboard.rs`) or per-character CGEvent (`inject/keyboard.rs`)
   - Append to composition buffer for potential AI rewrite

### Dual Transcription Backends

- **Local:** whisper-rs with Metal GPU acceleration. Models downloaded from Hugging Face, stored in app data dir.
- **Cloud:** Groq API (whisper-large-v3-turbo). Audio encoded as 16-bit WAV to halve upload size. Rate limit headers tracked for UI display.

### Composition Buffer & AI Rewrite

`state.rs` holds a `CompositionBuffer` — multi-segment accumulation of recent transcriptions (TTL 30min, max 20 entries). Tracks total injected character count. When rewrite is triggered (`rewrite.rs`), it:
1. Sends accumulated text to Groq Llama 3.3 70B with style prompt (Professional/Casual/Concise/Friendly)
2. Simulates backspace keystrokes to delete the exact character count
3. Injects rewritten text
4. Clears the buffer

### Window Management

Tray-only app — `tauri.conf.json` has `windows: []`, all windows created dynamically.

- **Overlay:** NSWindow swizzled to NSPanel at runtime (`windows.rs`) to render above full-screen Spaces. Window level 101 (NSPopUpMenuWindowLevel). Repositions to cursor's active monitor on each show. Click-through by default, toggles to interactive for rewrite prompt.
- **Settings:** Created on first tray menu click, reused on subsequent opens.
- **Welcome:** First-launch onboarding (permission requests, model download).

### State Management

- **Rust:** `AppState` struct in `state.rs` using `parking_lot::Mutex`/`RwLock` — recording state machine, audio buffer, WhisperEngine, SQLite pool, Groq usage, composition buffer, dictionary cache.
- **React:** `useSettings` hook for config, `useTauriEvents` for recording state and transcription results via Tauri event listeners.
- **IPC:** Tauri `invoke()` for commands (`src/lib/commands.ts` → `src-tauri/src/commands.rs`), `listen()` for async events.

## Conventions

- All Tauri commands defined in `commands.rs`, TypeScript wrappers in `src/lib/commands.ts`, types mirrored in `src/lib/types.ts`
- Use `parking_lot::Mutex`/`RwLock` instead of std (no lock poisoning)
- All audio processing in Rust — no Web Audio APIs on the frontend
- `macOSPrivateApi: true` in tauri.conf.json — required for NSPanel swizzle and transparent overlay
- CSP in tauri.conf.json allows `api.groq.com` and `api.lemonsqueezy.com`

## Data Storage

```
~/Library/Application Support/com.g1tech.shhhtype/
├── settings.json       # User preferences
├── dictionary.json     # Custom word corrections
├── shhhtype.db         # Transcription history (SQLite)
├── license.json        # LemonSqueezy license activation
├── .onboarding_complete
└── models/             # Downloaded Whisper .bin files
```

## Commercial Features

- **Licensing:** LemonSqueezy integration in `license.rs`. Machine ID derived from hashed macOS hardware UUID.
- **Groq usage tracking:** Rate limit headers parsed from API responses, displayed in Settings > License tab.

## macOS Permissions

- **Microphone** — required for audio capture (prompted on first launch)
- **Accessibility** — required for CGEvent text injection (must grant manually in System Settings)
