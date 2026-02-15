import { invoke } from "@tauri-apps/api/core";
import type {
  Settings,
  HistoryEntry,
  HistoryQuery,
  DictionaryEntry,
  PermissionStatus,
  RecordingState,
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
