/** Recording state from Rust backend */
export type RecordingState = "idle" | "recording" | "transcribing";

/** Transcription backend selection */
export type TranscriptionBackend = "Local" | "Cloud";

/** AI rewrite style */
export type RewriteStyle = "Professional" | "Casual" | "Concise" | "Friendly";

/** Overlay position */
export type OverlayPosition = "TopCenter" | "Inline";


/** License status */
export type LicenseStatus = "Trial" | "TrialExpired" | "Licensed" | "Beta" | "Invalid";

/** Trial information from Rust backend */
export interface TrialInfo {
  days_remaining: number;
  expired: boolean;
  message: string;
}

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
  overlay_position: OverlayPosition;
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

/** Update info from GitHub releases */
export interface UpdateInfo {
  tag_name: string;
  html_url: string;
  name: string;
}

/** Skill info from Rust backend */
export interface SkillInfo {
  name: string;
  trigger: string;
  aliases: string[];
  description: string;
}

/** Model download progress event payload */
export interface DownloadProgress {
  model: string;
  downloaded: number;
  total: number;
  percent: number;
}
