# vox2txt

A macOS menu bar voice-to-text tool for developers. Press a hotkey, speak, and your words are transcribed locally using Whisper and injected into the focused application.

All processing happens on-device via whisper.cpp with Metal GPU acceleration. No cloud APIs, no data leaves your machine.

## Features

- **Global hotkey** (Cmd+Shift+Space) with push-to-talk or toggle modes
- **Local Whisper transcription** with Metal GPU on Apple Silicon
- **Text injection** via clipboard paste or character-by-character keyboard simulation
- **Voice Activity Detection** - auto-stops recording after silence
- **Model management** - download Whisper models (Tiny to Large V3) from the Settings UI
- **Custom dictionary** - correct terms Whisper frequently gets wrong
- **Searchable history** with export to JSON
- **Floating overlay** during recording
- **Sound feedback** on start/stop
- **macOS notifications** on transcription complete
- **Launch at login** support
- **10 languages** + auto-detect

## Prerequisites

- **macOS** (Apple Silicon recommended for Metal GPU acceleration)
- **Rust** (via [rustup](https://rustup.rs/))
- **Node.js** 18+ and **npm**
- **Xcode Command Line Tools** (`xcode-select --install`)

## Setup

```bash
# Clone the repository
git clone https://github.com/genesis1tech/ts_vision.git
cd ts_vision/vox2txt

# Install frontend dependencies
npm install
```

## Development

```bash
# Launch in dev mode with hot-reload and Rust logging
RUST_LOG=info npm run tauri dev
```

This starts both the Vite dev server (frontend) and the Tauri Rust backend. The app appears as a system tray icon.

## Building for Production

```bash
# Build the .app bundle and .dmg installer
npm run tauri build
```

The built application will be in `src-tauri/target/release/bundle/`:
- `macos/vox2txt.app` - The application bundle (drag to /Applications)
- `dmg/vox2txt_0.1.0_aarch64.dmg` - Disk image installer for distribution

## Installing on Other Macs

**Option A: Share the `.dmg` (simplest)**

1. Build the app with `npm run tauri build`
2. Send the `.dmg` file from `src-tauri/target/release/bundle/dmg/` to the other Mac
3. Open the `.dmg`, drag `vox2txt.app` to `/Applications`
4. On first launch, right-click > Open (since it's unsigned, macOS will warn about unidentified developer)
5. Grant Microphone and Accessibility permissions when prompted

**Option B: Code-signed `.dmg` (no Gatekeeper warnings)**

To distribute without the "unidentified developer" warning:

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
   xcrun notarytool submit src-tauri/target/release/bundle/dmg/vox2txt_0.1.0_aarch64.dmg \
     --apple-id your@email.com --team-id TEAM_ID --password app-specific-password --wait
   xcrun stapler staple src-tauri/target/release/bundle/dmg/vox2txt_0.1.0_aarch64.dmg
   ```
5. The signed+notarized `.dmg` can be shared and opens without warnings

**Note:** The build is architecture-specific. An Apple Silicon Mac produces an `aarch64` build. For Intel Macs, build on an Intel Mac or use `--target x86_64-apple-darwin`.

## First Launch

1. **Grant permissions** - macOS will prompt for Microphone and Accessibility access. Both are required.
2. **Download a model** - Open Settings (tray icon > Settings), go to the General tab, select a model size, and click Download. Start with **Base (142MB)** for a good speed/accuracy balance.
3. **Press the hotkey** - Default is `Cmd+Shift+Space`. Speak, then release (push-to-talk) or press again (toggle mode). Your transcription is typed into the focused app.

## Configuration

Open Settings from the system tray menu. Available options:

| Setting | Tab | Description |
|---------|-----|-------------|
| Whisper Model | General | Tiny through Large V3 Turbo |
| Language | General | English, Spanish, French, etc. or auto-detect |
| Hotkey | General | Customizable keyboard shortcut |
| Mode | General | Push-to-talk or toggle |
| Injection Method | General | Clipboard paste (Cmd+V) or keyboard simulation |
| Auto-copy | General | Copy transcription to clipboard when using keyboard injection |
| Launch at login | General | Start vox2txt on macOS login |
| VAD Threshold | Audio | Silence detection sensitivity |
| Show Overlay | Audio | Floating recording indicator |
| Sound Feedback | Audio | Audible beep on start/stop |
| Dictionary | Dictionary | Custom word corrections |
| History | History | Search, review, and export past transcriptions |

## Tech Stack

- **Frontend**: React 19, TypeScript, Tailwind CSS 4, Vite 7
- **Backend**: Rust, Tauri v2
- **STT Engine**: whisper-rs (whisper.cpp bindings) with Metal GPU
- **Audio**: cpal (capture), rubato (resampling)
- **Database**: SQLite via rusqlite
- **Text Injection**: Core Graphics CGEvent API

## Data Storage

All data is stored locally at:
```
~/Library/Application Support/com.g1tech.vox2txt/
  settings.json      # User preferences
  dictionary.json    # Custom word corrections
  vox2txt.db       # Transcription history (SQLite)
  models/            # Downloaded Whisper model files
```

## License

Private.
