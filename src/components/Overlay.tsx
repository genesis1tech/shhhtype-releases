import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { useRecordingState } from "../hooks/useTauriEvents";
import { rewriteLastTranscription } from "../lib/commands";

type OverlayMode = "idle" | "recording" | "transcribing" | "rewrite-prompt" | "rewriting" | "rewrite-done";

/** Floating transparent overlay showing recording status and rewrite prompt. */
export default function Overlay() {
  const recordingState = useRecordingState();
  const [mode, setMode] = useState<OverlayMode>("idle");
  const [rewriteTimer, setRewriteTimer] = useState<ReturnType<typeof setTimeout> | null>(null);

  // Sync recording state to overlay mode
  useEffect(() => {
    if (recordingState === "recording") {
      setMode("recording");
      if (rewriteTimer) clearTimeout(rewriteTimer);
    } else if (recordingState === "transcribing" && mode !== "rewriting") {
      setMode("transcribing");
    }
  }, [recordingState]);

  // Listen for transcription complete — show rewrite prompt briefly
  useEffect(() => {
    const unlistenComplete = listen("transcription-complete", () => {
      setMode("rewrite-prompt");
      const timer = setTimeout(() => {
        setMode((m) => (m === "rewrite-prompt" ? "idle" : m));
      }, 3000);
      setRewriteTimer(timer);
    });

    const unlistenRewriteStart = listen("rewrite-started", () => {
      setMode("rewriting");
      if (rewriteTimer) clearTimeout(rewriteTimer);
    });

    const unlistenRewriteDone = listen("rewrite-complete", () => {
      setMode("rewrite-done");
      setTimeout(() => setMode("idle"), 1500);
    });

    const unlistenRewriteError = listen("rewrite-error", () => {
      setMode("idle");
    });

    return () => {
      unlistenComplete.then((fn) => fn());
      unlistenRewriteStart.then((fn) => fn());
      unlistenRewriteDone.then((fn) => fn());
      unlistenRewriteError.then((fn) => fn());
    };
  }, []);

  const handleRewrite = async () => {
    if (rewriteTimer) clearTimeout(rewriteTimer);
    setMode("rewriting");
    try {
      await rewriteLastTranscription();
      setMode("rewrite-done");
      setTimeout(() => setMode("idle"), 1500);
    } catch (e) {
      console.error("Rewrite failed:", e);
      setMode("idle");
    }
  };

  if (mode === "idle") {
    return <div className="overlay-window h-screen" />;
  }

  return (
    <div className="overlay-window flex items-center justify-center h-screen">
      <div className="flex items-center gap-3 rounded-full bg-black/80 px-5 py-3 shadow-2xl backdrop-blur-sm">
        {mode === "recording" && (
          <>
            <span className="inline-block h-3 w-3 rounded-full bg-red-500 animate-pulse" />
            <span className="text-sm font-medium text-white">Listening...</span>
          </>
        )}

        {mode === "transcribing" && (
          <>
            <span className="inline-block h-3 w-3 rounded-full bg-yellow-400 animate-pulse" />
            <span className="text-sm font-medium text-white">Transcribing...</span>
          </>
        )}

        {mode === "rewrite-prompt" && (
          <>
            <span className="inline-block h-3 w-3 rounded-full bg-green-400" />
            <span className="text-sm font-medium text-white">Done</span>
            <button
              onClick={handleRewrite}
              className="ml-1 bg-blue-600 hover:bg-blue-700 text-white text-xs font-medium px-3 py-1 rounded-full transition-colors"
            >
              Rewrite?
            </button>
          </>
        )}

        {mode === "rewriting" && (
          <>
            <span className="inline-block h-3 w-3 rounded-full bg-purple-400 animate-pulse" />
            <span className="text-sm font-medium text-white">Rewriting...</span>
          </>
        )}

        {mode === "rewrite-done" && (
          <>
            <span className="inline-block h-3 w-3 rounded-full bg-green-400" />
            <span className="text-sm font-medium text-white">Rewritten</span>
          </>
        )}
      </div>
    </div>
  );
}
