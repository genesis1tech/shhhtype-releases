/** Recording state from Rust backend */
export type RecordingState = "idle" | "recording" | "transcribing";

/** Settings matching Rust Settings struct */
export interface Settings {
  model_size: ModelSize;
  shortcut: string;
  hotkey_mode: HotkeyMode;
  injection_method: InjectionMethod;
  language: string;
  auto_copy: boolean;
  vad_threshold: number;
  show_overlay: boolean;
  sound_feedback: boolean;
  auto_launch: boolean;
  transcription_backend: TranscriptionBackend;
  groq_api_key?: string;
}

export type ModelSize =
  | "Tiny"
  | "Base"
  | "Small"
  | "Medium"
  | "LargeV3"
  | "LargeV3Turbo";

export type HotkeyMode = "PushToTalk" | "Toggle";

export type InjectionMethod = "Clipboard" | "Keyboard";

export type TranscriptionBackend = "Local" | "Groq";

/** History entry matching Rust HistoryEntry struct */
export interface HistoryEntry {
  id: string;
  text: string;
  duration_ms: number;
  model: string;
  created_at: string;
  app_name: string | null;
  word_count: number;
}

/** History query parameters */
export interface HistoryQuery {
  search?: string;
  limit?: number;
  offset?: number;
}

/** Dictionary entry matching Rust DictionaryEntry struct */
export interface DictionaryEntry {
  from: string;
  to: string;
}

/** Permission status from macOS */
export interface PermissionStatus {
  microphone: boolean;
  accessibility: boolean;
}

/** Model download/availability status */
export interface ModelStatus {
  model: string;
  downloaded: boolean;
  size_bytes: number | null;
}

/** Model download progress event payload */
export interface DownloadProgress {
  model: string;
  downloaded: number;
  total: number;
  percent: number;
}
