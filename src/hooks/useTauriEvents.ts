import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import type { RecordingState } from "../lib/types";

/** Listen to recording state changes from the Rust backend. */
export function useRecordingState() {
  const [state, setState] = useState<RecordingState>("idle");

  useEffect(() => {
    const unlisten = listen<RecordingState>("recording-state-changed", (event) => {
      setState(event.payload);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  return state;
}

/** Listen to transcription results from the Rust backend. */
export function useTranscriptionResult() {
  const [result, setResult] = useState<string | null>(null);

  useEffect(() => {
    const unlisten = listen<string>("transcription-complete", (event) => {
      setResult(event.payload);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  return result;
}
