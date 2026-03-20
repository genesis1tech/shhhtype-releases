# Transcription Pipeline Optimizations

Applied to the macOS version on 2026-03-19. Port these to the Windows version.

## 1. Skip Resampling for Cloud Transcription (BIGGEST WIN)

**Before:** 48kHz audio -> resample to 16kHz (3+ seconds) -> encode WAV -> upload to Groq
**After:** 48kHz audio -> encode WAV at native rate -> upload to Groq

Groq accepts any sample rate and downsamples server-side. Resampling is only needed for local Whisper.

### Changes
- `groq.rs`: `encode_wav()` now accepts a `sample_rate` parameter instead of hardcoding 16000
- `groq.rs`: `transcribe()` now accepts `sample_rate` and passes raw audio directly
- `commands.rs`: Cloud path sends `raw_samples` at native `sample_rate`; Local path still resamples to 16kHz

## 2. Use distil-whisper for English (Faster + Cheaper)

**Model selection by language:**
- English (`en`): `distil-whisper-large-v3-en` — 24x faster, 50% cheaper, English-only
- All other languages: `whisper-large-v3-turbo` — multilingual support

### Changes
- `groq.rs`: Model selected based on `language` parameter:
  ```rust
  let model = if language == "en" {
      "distil-whisper-large-v3-en"
  } else {
      "whisper-large-v3-turbo"
  };
  ```

## 3. Reuse HTTP Client (Save TLS Handshake)

**Before:** New `reqwest::blocking::Client` created per request
**After:** Static `LazyLock<Client>` reused across all requests

### Changes
- `groq.rs`: Added `static HTTP_CLIENT: LazyLock<reqwest::blocking::Client>`
- All API calls use `HTTP_CLIENT` instead of creating a new client

## 4. WAV Encoding: 32-bit Float -> 16-bit Int

Halves upload payload size with zero quality loss for speech.

### Changes
- `groq.rs`: `encode_wav()` uses `bits_per_sample: 16` + `SampleFormat::Int`
- Converts `f32` samples to `i16` with clamping

## 5. Zero-Copy Audio Buffer Extraction

**Before:** `buf.clone()` + `buf.clear()` — duplicates entire audio buffer
**After:** `std::mem::take(&mut *buf)` — moves ownership, no copy

### Changes
- `commands.rs`: `do_stop_and_transcribe()` uses `mem::take` instead of clone+clear

## 6. Dictionary Caching

**Before:** `dictionary.json` loaded from disk on every transcription
**After:** Cached in `AppState`, invalidated only when user updates dictionary

### Changes
- `state.rs`: Added `dictionary_cache: Mutex<Option<Dictionary>>` to `AppState`
- `commands.rs`: Uses `cache.get_or_insert_with()` for lazy loading
- `commands.rs`: `update_dictionary()` sets cache to `None` to invalidate

## 7. Keyboard Injection Speed

**Before:** 5ms sleep between each keystroke
**After:** 2ms sleep between each keystroke

~60% faster text appearance for keyboard injection method.

### Changes
- `inject/keyboard.rs`: Sleep reduced from 5ms to 2ms

## 8. Resampler Parameters (For Local Whisper Only)

Reduced sinc interpolation parameters — overkill for speech audio:
- `sinc_len`: 256 -> 64
- `oversampling_factor`: 256 -> 128

### Changes
- `audio/resampler.rs`: Updated `SincInterpolationParameters`

## 9. CGEvent Tap Location Consistency

Standardized keyboard injection to use `AnnotatedSession` tap location (matches clipboard injection). More reliable on recent macOS versions.

### Changes
- `inject/keyboard.rs`: Changed `HID` to `AnnotatedSession` for key_down and key_up posts

## Performance Impact Summary

| Optimization | Latency Saved |
|---|---|
| Skip resampling (Cloud) | ~3 seconds |
| distil-whisper model | ~10-20% faster API |
| HTTP client reuse | ~100-200ms |
| 16-bit WAV encoding | ~50% smaller upload |
| Zero-copy buffer | ~10-50ms |
| Dictionary cache | ~1-2ms |
| Keyboard injection | ~60% faster typing |
| Resampler params | ~50% faster (local only) |
