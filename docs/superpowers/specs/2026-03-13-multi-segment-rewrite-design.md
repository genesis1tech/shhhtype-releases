# Multi-Segment Rewrite

## Context

Currently, the AI rewrite feature only operates on the single most recent transcription (`last_transcription` in AppState). Users composing longer content — letters, emails, multi-paragraph messages — dictate across multiple recording sessions but can only rewrite the last segment. They need a way to accumulate multiple transcriptions and rewrite them as one cohesive piece.

## Design

### Concept: Always-Accumulating Composition Buffer

Every transcription automatically appends to an in-memory buffer. The rewrite hotkey operates on the full buffer contents. No explicit "start session" action is needed — the buffer is invisible until you rewrite.

### Buffer Rules

- **In-memory only** — `Vec<String>` in AppState, not persisted to disk. Starts empty on app launch.
- **Auto-clear on rewrite** — After a successful rewrite, the buffer clears.
- **TTL (30 minutes)** — If 30+ minutes have passed since the last transcription, the next recording clears the buffer before appending. This prevents stale context from leaking across unrelated tasks.
- **Max 20 entries** — If the buffer reaches 20 entries, the oldest entry is dropped (sliding window). Prevents unbounded growth for users who never rewrite.
- **Manual clear** — Tray menu item "Clear Composition" lets users explicitly reset the buffer.

### Rewrite Behavior

- **Single entry in buffer** — Behaves exactly like today: Cmd+Z to undo the last paste, then inject rewritten text. No visible change for existing users.
- **Multiple entries in buffer** — Entries are joined with spaces into one text block, sent to Groq for rewrite. The rewritten text is **copied to clipboard** and a notification tells the user. No Cmd+Z attempt (can't reliably undo multiple separate pastes across apps).
- **Buffer clears after rewrite** regardless of single/multi mode.

### Overlay Feedback

After each transcription, the overlay briefly shows a segment counter: e.g., "2 segments" next to the status text. This gives the user awareness of buffer state without any extra UI.

### Tray Menu Addition

Add a "Clear Composition" item to the tray menu (only visible when the buffer has entries). Clears the buffer and shows a brief notification.

## Key Files

| File | Change |
|------|--------|
| `src-tauri/src/state.rs` | Add `CompositionBuffer` struct with `Vec<String>`, `last_appended_at: Option<Instant>`, methods for append/clear/join/len. Add field to `AppState`. |
| `src-tauri/src/commands.rs` | Update `do_stop_and_transcribe` to append to buffer. Update `rewrite_last_transcription` to read from buffer, use clipboard strategy for multi. Add `clear_composition` and `get_composition_count` commands. |
| `src-tauri/src/lib.rs` | Update hotkey rewrite handler to use buffer. Register new commands. Emit segment count with transcription-complete event. |
| `src-tauri/src/tray/setup.rs` | Add "Clear Composition" menu item. |
| `src/lib/types.ts` | No new types needed (count is just a number). |
| `src/lib/commands.ts` | Add `clearComposition()` and `getCompositionCount()` wrappers. |
| `src/components/Overlay.tsx` | Show segment count after transcription completes. |

## Verification

1. `cargo check` passes
2. `npx tsc --noEmit` passes
3. Dictate once, hit rewrite hotkey — behaves identically to today (undo+replace)
4. Dictate 3 times in quick succession, hit rewrite — all 3 segments combined and rewritten, result copied to clipboard, notification shown
5. Wait 30+ minutes, dictate again — buffer has only 1 entry (TTL cleared old entries)
6. Dictate 21 times without rewriting — buffer has 20 entries (oldest dropped)
7. Use "Clear Composition" from tray menu — buffer empties, overlay shows no segment count
8. User who never rewrites — buffer self-manages, no memory growth, no impact on normal flow
