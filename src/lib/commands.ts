import { invoke } from "@tauri-apps/api/core";
import type {
  Settings,
  HistoryEntry,
  HistoryQuery,
  DictionaryEntry,
  PermissionStatus,
  RecordingState,
  ModelStatus,
  ModelSize,
  RewriteStyle,
  LicenseStatus,
  GroqUsage,
  AudioDevice,
  UpdateInfo,
} from "./types";

/** Start audio recording */
export const startRecording = () => invoke<void>("start_recording");

/** Stop recording and get transcription */
export const stopRecording = () => invoke<string>("stop_recording");

/** Cancel recording without transcribing */
export const cancelRecording = () => invoke<void>("cancel_recording");

/** Get current recording state */
export const getRecordingState = () =>
  invoke<RecordingState>("get_recording_state");

/** Get application settings */
export const getSettings = () => invoke<Settings>("get_settings");

/** Update application settings */
export const updateSettings = (settings: Settings) =>
  invoke<void>("update_settings", { settings });

/** Query transcription history */
export const getHistory = (query: HistoryQuery) =>
  invoke<HistoryEntry[]>("get_history", { query });

/** Delete a history entry */
export const deleteHistoryEntry = (id: string) =>
  invoke<void>("delete_history_entry", { id });

/** Get custom dictionary entries */
export const getDictionary = () =>
  invoke<DictionaryEntry[]>("get_dictionary");

/** Update custom dictionary */
export const updateDictionary = (entries: DictionaryEntry[]) =>
  invoke<void>("update_dictionary", { entries });

/** Check macOS permissions */
export const checkPermissions = () =>
  invoke<PermissionStatus>("check_permissions");

/** Request microphone permission (triggers macOS prompt) */
export const requestMicrophonePermission = () =>
  invoke<void>("request_microphone_permission");

/** Get download status of all models */
export const getModelStatus = () =>
  invoke<ModelStatus[]>("get_model_status");

/** Download a Whisper model from Hugging Face */
export const downloadModel = (modelSize: ModelSize) =>
  invoke<void>("download_model", { modelSize });

/** Delete a downloaded model */
export const deleteModel = (modelSize: ModelSize) =>
  invoke<void>("delete_model", { modelSize });

/** Export all history entries */
export const exportHistory = () =>
  invoke<HistoryEntry[]>("export_history");

/** Rewrite last transcription using AI */
export const rewriteLastTranscription = (style?: RewriteStyle) =>
  invoke<{ text: string; is_multi: boolean }>("rewrite_last_transcription", { style: style ?? null });

/** Rewrite and inject: rewrites composition text, then selects-back and replaces in the target app */
export const rewriteAndInject = (style?: RewriteStyle) =>
  invoke<{ text: string; is_multi: boolean }>("rewrite_and_inject", { style: style ?? null });

/** Get Groq API rate limit usage */
export const getGroqUsage = () => invoke<GroqUsage>("get_groq_usage");

/** Activate a license key */
export const activateLicense = (key: string) =>
  invoke<LicenseStatus>("activate_license", { key });

/** Get current license status */
export const getLicenseStatus = () =>
  invoke<LicenseStatus>("get_license_status");

/** Deactivate current license */
export const deactivateLicense = () =>
  invoke<void>("deactivate_license");

/** Clear the composition buffer */
export const clearComposition = () => invoke<void>("clear_composition");

/** Get number of segments in composition buffer */
export const getCompositionCount = () =>
  invoke<number>("get_composition_count");

/** List available audio input devices */
export const listAudioDevices = () =>
  invoke<AudioDevice[]>("list_audio_devices");

/** Restart the app (to apply hotkey changes) */
export const restartApp = () => invoke<void>("restart_app");

/** Check GitHub for available updates */
export const checkForUpdates = () =>
  invoke<UpdateInfo | null>("check_for_updates");

/** Get cached update info (if a newer version was found) */
export const getUpdateInfo = () =>
  invoke<UpdateInfo | null>("get_update_info");
