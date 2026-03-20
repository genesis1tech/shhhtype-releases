/** Recording state from Rust backend */
export type RecordingState = "idle" | "recording" | "transcribing";

/** Transcription backend selection */
export type TranscriptionBackend = "Local" | "Cloud";

/** AI rewrite style */
export type RewriteStyle = "Professional" | "Casual" | "Concise" | "Friendly";


/** License status */
export type LicenseStatus = "Free" | "Licensed" | "Invalid";

/** Settings matching Rust Settings struct */
export interface Settings {
  model_size: ModelSize;
  shortcut: string;
  hotkey_mode: HotkeyMode;
  injection_method: InjectionMethod;
  language: string;
  auto_copy: boolean;
  vad_threshold: number;
  vad_silence_timeout: number;
  show_overlay: boolean;
  sound_feedback: boolean;
  auto_launch: boolean;
  transcription_backend: TranscriptionBackend;
  groq_api_key: string | null;
  rewrite_enabled: boolean;
  rewrite_style: RewriteStyle;
  rewrite_hotkey: string;
  audio_input_device: string | null;
}

/** Audio input device info */
export interface AudioDevice {
  name: string;
  is_default: boolean;
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

/** Groq API rate limit usage */
export interface GroqUsage {
  limit_requests: number | null;
  remaining_requests: number | null;
  reset_requests: string | null;
  limit_tokens: number | null;
  remaining_tokens: number | null;
  reset_tokens: string | null;
  updated_at: string | null;
}

/** Model download progress event payload */
export interface DownloadProgress {
  model: string;
  downloaded: number;
  total: number;
  percent: number;
}
