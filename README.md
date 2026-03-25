# ShhhType

A voice-to-text desktop app that turns your speech into polished, publish-ready content. Press a hotkey, speak, and ShhhType transcribes your words and rewrites them using AI — LinkedIn posts, DMs, connection notes, and more. Works on macOS and Windows.

**The killer feature:** Voice-triggered skills. Say "slash linkedin" and your spoken thoughts become a formatted LinkedIn post with bold text, spacing, and hashtags. Say "slash dm" and get a personalized direct message. Say "slash connect" and get a connection request note under LinkedIn's 200-character limit.

Built for LinkedIn content creators, founders, and professionals who have ideas but hate typing them out.

## Voice Skills

ShhhType's skill system transforms raw speech into polished content for specific use cases. Say the trigger command at the start or end of your recording.

| Skill | Trigger | What It Does |
|-------|---------|-------------|
| **LinkedIn Post** | `/linkedin` or `/social` | Polished post with bold/italic Unicode formatting, HVCTA structure, hashtags |
| **LinkedIn DM** | `/dm` | Personalized direct message (6 types: cold, warm, congrats, collab, follow-up, referral). Under 300 chars with anti-spam guardrails |
| **Connection Note** | `/connect` | Connection request note under LinkedIn's 200-character limit |
| **Hormozi Style** | `/hormozi` | Content in Alex Hormozi's punchy, framework-driven voice |

Skills are triggered by voice — say "slash linkedin" or type `/linkedin` at the start or end of your recording. View all skills in Settings > Skills tab.

## Transcription

Two transcription backends:

- **Local mode** — fully on-device via whisper.cpp with Metal GPU acceleration. No cloud APIs, no data leaves your machine.
- **Cloud mode** — optional [Groq](https://groq.com/) backend using whisper-large-v3-turbo for faster transcription. Requires a free Groq API key.

## Features

- **Global hotkey** (`Cmd+Alt+V` default) with push-to-talk or toggle modes
- **Local Whisper transcription** with Metal GPU acceleration on Apple Silicon
- **Groq cloud transcription** as an optional backend (whisper-large-v3-turbo)
- **AI rewrite** — rewrite transcribed text using Groq Qwen3 32B with 4 styles (Professional, Casual, Concise, Friendly) via `Cmd+Alt+R`
- **Voice-triggered skills** — say "slash linkedin" to transform speech into formatted LinkedIn posts, DMs, connection notes, and more
- **Composition buffer** — accumulates multiple transcription segments (30min TTL) so rewrites can span across recordings
- **Text injection** into any focused application via clipboard paste (`Cmd+V`) or character-by-character keyboard simulation
- **Voice Activity Detection** — auto-stops recording after configurable silence timeout
- **Model management** — download Whisper models (Tiny 75MB to Large V3 3.1GB) from the Settings UI
- **Custom dictionary** — correct terms Whisper frequently gets wrong
- **Skills tab** in Settings — view all loaded skills with triggers and descriptions
- **Searchable history** with export to JSON
- **Floating overlay** pill indicator during recording (follows cursor across monitors, renders over full-screen apps)
- **Sound feedback** on start/stop
- **macOS notifications** on transcription complete
- **Launch at login** support
- **9 languages** + auto-detect (English, Spanish, French, German, Italian, Portuguese, Japanese, Korean, Chinese)
- **7-day free trial** with full feature access, then license activation via LemonSqueezy

## Prerequisites

- **macOS** (Apple Silicon recommended for Metal GPU acceleration)
- **Rust** (via [rustup](https://rustup.rs/))
- **Node.js** 18+ and **npm**
- **Xcode Command Line Tools** (`xcode-select --install`)
- **cmake** (required for building whisper-sys: `brew install cmake`)

## Setup

```bash
# Clone the repository
git clone https://github.com/genesis1tech/shhhtype.git
cd shhhtype

# Install frontend dependencies
npm install
```

## Development

```bash
# Launch in dev mode with hot-reload and Rust logging
RUST_LOG=info npm run tauri dev
```

This starts both the Vite dev server (frontend) and the Tauri Rust backend. The app appears as a system tray icon.

### Useful commands

```bash
# Check Rust only (faster feedback loop)
cargo check --manifest-path src-tauri/Cargo.toml

# Build Rust only
cargo build --manifest-path src-tauri/Cargo.toml

# Build frontend only
npx vite build
```

## Building for Production

```bash
npm run tauri build
```

The built application will be in `src-tauri/target/release/bundle/`:
- `macos/ShhhType.app` — application bundle (drag to `/Applications`)
- `dmg/ShhhType_0.1.0_aarch64.dmg` — disk image installer

## Installing on Other Macs

**Option A: Share the `.dmg` (simplest)**

1. Build with `npm run tauri build`
2. Send the `.dmg` from `src-tauri/target/release/bundle/dmg/`
3. Open the `.dmg`, drag `ShhhType.app` to `/Applications`
4. On first launch, right-click > Open (macOS will warn about unidentified developer)
5. Grant Microphone and Accessibility permissions when prompted

**Option B: Code-signed `.dmg` (no Gatekeeper warnings)**

1. Get an [Apple Developer ID](https://developer.apple.com/account/) ($99/year)
2. Add your signing identity to `src-tauri/tauri.conf.json`:
   ```json
   "macOS": {
     "signingIdentity": "Developer ID Application: Your Name (TEAM_ID)"
   }
   ```
3. Build: `npm run tauri build`
4. Notarize (required for macOS 10.15+):
   ```bash
   xcrun notarytool submit src-tauri/target/release/bundle/dmg/ShhhType_0.1.0_aarch64.dmg \
     --apple-id your@email.com --team-id TEAM_ID --password app-specific-password --wait
   xcrun stapler staple src-tauri/target/release/bundle/dmg/ShhhType_0.1.0_aarch64.dmg
   ```

**Note:** The build is architecture-specific. Apple Silicon produces `aarch64`. For Intel Macs, build on Intel or use `--target x86_64-apple-darwin`.

## First Launch

1. **Grant permissions** — macOS will prompt for Microphone and Accessibility access. Both are required.
2. **Download a model** — Open Settings (tray icon > Settings), go to the General tab, select a model size, and click Download. Start with **Base (142MB)** for a good speed/accuracy balance.
3. **Press the hotkey** — Default is `Cmd+Alt+V`. Speak, then release (push-to-talk) or press again (toggle mode). Your transcription is typed into the focused app.

## Configuration

Open Settings from the system tray menu.

| Setting | Tab | Description |
|---------|-----|-------------|
| Whisper Model | General | Tiny, Base, Small, Medium, Large V3, Large V3 Turbo |
| Transcription Backend | General | Local (on-device Whisper) or Cloud (Groq API) |
| Groq API Key | General | Required for cloud transcription and AI rewrite |
| Language | General | English, Spanish, French, German, Italian, Portuguese, Japanese, Korean, Chinese, or auto-detect |
| Hotkey | General | Customizable keyboard shortcut (click Change, press new combo) |
| Mode | General | Push-to-talk (hold key) or toggle (press to start/stop) |
| Injection Method | General | Clipboard paste (`Cmd+V`) or keyboard simulation |
| Auto-copy | General | Copy transcription to clipboard when using keyboard injection |
| Launch at login | General | Start ShhhType on macOS login |
| AI Rewrite | General | Enable rewrite, choose style (Professional/Casual/Concise/Friendly), set rewrite hotkey |
| VAD Threshold | Audio | Silence detection sensitivity (lower = more sensitive) |
| Show Overlay | Audio | Floating recording indicator pill |
| Sound Feedback | Audio | Audible beep on start/stop |
| Dictionary | Dictionary | Custom word corrections (e.g., "react native" to "React Native") |
| Skills | Skills | View all loaded voice skills with triggers and descriptions |
| History | History | Search, review, and export past transcriptions |
| License | License | Activate/deactivate license key, view Groq API usage and rate limits |

## Architecture

```
Menu Bar Tray App (no main window)
├── Global Hotkey (push-to-talk / toggle)
├── Audio Pipeline: cpal capture → rubato resample 16kHz → VAD silence detection → buffer
├── Transcription: whisper-rs + Metal GPU → dictionary corrections
│   └── Optional: Groq cloud API (whisper-large-v3-turbo)
├── Voice Skills: /linkedin, /dm, /connect, /hormozi (trigger detection at start/end of speech)
├── AI Rewrite: Groq Qwen3 32B with composition buffer (multi-segment accumulation)
│   └── Unicode formatting: markdown bold/italic → Unicode Math Bold/Italic for LinkedIn
├── Text Injection: clipboard paste (arboard + CGEvent Cmd+V) or keyboard simulation (CGEvent)
├── Overlay Window: transparent NSPanel pill (renders over full-screen apps, follows cursor across monitors)
└── Settings Window: tabbed config panel (General, Audio, Dictionary, Skills, History, License, About)
```

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Framework | Tauri v2 (Rust backend + webview frontend) |
| Frontend | React 19, TypeScript, Tailwind CSS 4, Vite 7 |
| STT Engine | whisper-rs (whisper.cpp bindings) with Metal GPU |
| Cloud STT | Groq API (optional, whisper-large-v3-turbo) |
| AI Rewrite | Groq Qwen3 32B |
| Audio | cpal (capture), rubato (resampling), hound (WAV encoding) |
| Text Injection | arboard (clipboard), core-graphics (CGEvent API) |
| Database | SQLite via rusqlite (bundled) |
| Licensing | LemonSqueezy API |
| Hotkeys | tauri-plugin-global-shortcut |

## Project Structure

```
src-tauri/src/
├── lib.rs              # App setup, hotkey registration, event loop
├── commands.rs         # Tauri IPC commands (start/stop recording, settings, history)
├── state.rs            # Shared application state (AppState, CompositionBuffer)
├── audio/
│   ├── capture.rs      # Microphone capture via cpal
│   └── resampler.rs    # Resample to 16kHz for Whisper
├── transcribe/
│   ├── engine.rs       # Whisper model wrapper
│   ├── model.rs        # Model management (download, delete, status)
│   ├── dictionary.rs   # Custom word corrections
│   └── groq.rs         # Groq cloud transcription backend
├── inject/
│   ├── clipboard.rs    # Clipboard paste injection (Cmd+V)
│   └── keyboard.rs     # Character-by-character keyboard simulation
├── config/
│   └── settings.rs     # Settings persistence (JSON)
├── db/
│   ├── history.rs      # Transcription history CRUD
│   └── migrations.rs   # SQLite schema migrations
├── hotkey/
│   └── manager.rs      # Hotkey mode definitions
├── vad/
│   └── energy.rs       # Energy-based voice activity detection
├── tray/
│   └── setup.rs        # System tray icon and menu
├── windows.rs          # Overlay/settings window management, NSPanel swizzle
├── rewrite.rs          # AI rewrite via Groq Qwen3 32B + markdown→Unicode conversion
├── skills.rs           # Voice skill system (.md files with YAML frontmatter, trigger detection)
├── license.rs          # LemonSqueezy license activation
└── sound.rs            # Audio feedback (start/stop beeps)

src/
├── components/
│   ├── Settings.tsx     # Tabbed settings panel
│   ├── History.tsx      # Searchable transcription history
│   ├── Overlay.tsx      # Recording status overlay with rewrite prompt
│   ├── Welcome.tsx      # First-launch onboarding flow
│   └── PermissionStatus.tsx
├── hooks/
│   ├── useSettings.ts   # Settings state management
│   ├── useHistory.ts    # History query and export
│   └── useTauriEvents.ts # Recording state and transcription event listeners
└── lib/
    ├── commands.ts      # Tauri IPC bindings (TypeScript)
    └── types.ts         # TypeScript types mirroring Rust structs
```

## Data Storage

All data is stored locally. When using the Groq cloud backend, audio is sent to Groq's API for transcription — no other data leaves your machine.

```
~/Library/Application Support/com.g1tech.shhhtype/
├── settings.json       # User preferences
├── dictionary.json     # Custom word corrections
├── shhhtype.db         # Transcription history (SQLite)
├── license.json        # LemonSqueezy license activation
├── skills/             # Voice skill .md files (linkedin, dm, connect, hormozi)
└── models/             # Downloaded Whisper model files (.bin)
```

## Permissions

ShhhType requires two macOS permissions:

- **Microphone** — for audio capture. The app will prompt on first launch.
- **Accessibility** — for text injection via keyboard simulation. Grant in System Settings > Privacy & Security > Accessibility.

The Settings window shows a banner when permissions are missing, with a button to trigger the system prompt.

## License

MIT
