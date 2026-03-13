import { useEffect, useState, useRef } from "react";
import { listen } from "@tauri-apps/api/event";
import { useRecordingState } from "../hooks/useTauriEvents";
import { rewriteLastTranscription } from "../lib/commands";

type OverlayMode = "idle" | "recording" | "transcribing" | "rewrite-prompt" | "rewriting" | "rewrite-done";

/** Animated waveform bars driven by audio levels from the backend. */
/** Map level (0–1) to a color: green → cyan → blue → purple → red */
function levelColor(level: number): string {
  if (level < 0.25) {
    // Green to cyan
    const t = level / 0.25;
    const r = Math.round(34 + t * (6 - 34));
    const g = Math.round(197 + t * (182 - 197));
    const b = Math.round(94 + t * (212 - 94));
    return `rgb(${r},${g},${b})`;
  } else if (level < 0.5) {
    // Cyan to blue
    const t = (level - 0.25) / 0.25;
    const r = Math.round(6 + t * (59 - 6));
    const g = Math.round(182 + t * (130 - 182));
    const b = Math.round(212 + t * (246 - 212));
    return `rgb(${r},${g},${b})`;
  } else if (level < 0.75) {
    // Blue to purple/pink
    const t = (level - 0.5) / 0.25;
    const r = Math.round(59 + t * (217 - 59));
    const g = Math.round(130 + t * (70 - 130));
    const b = Math.round(246 + t * (239 - 246));
    return `rgb(${r},${g},${b})`;
  } else {
    // Purple to red
    const t = (level - 0.75) / 0.25;
    const r = Math.round(217 + t * (248 - 217));
    const g = Math.round(70 + t * (68 - 70));
    const b = Math.round(239 + t * (73 - 239));
    return `rgb(${r},${g},${b})`;
  }
}

function Waveform({ levels }: { levels: number[] }) {
  return (
    <div className="flex items-center gap-[2px] h-7">
      {levels.map((level, i) => (
        <div
          key={i}
          className="w-[2.5px] rounded-full"
          style={{
            height: `${Math.max(3, level * 28)}px`,
            backgroundColor: levelColor(level),
            opacity: 0.7 + level * 0.3,
          }}
        />
      ))}
    </div>
  );
}

/** Floating transparent overlay showing recording status and rewrite prompt. */
export default function Overlay() {
  const recordingState = useRecordingState();
  const [mode, setMode] = useState<OverlayMode>("idle");
  const [segmentCount, setSegmentCount] = useState(0);
  const [rewriteTimer, setRewriteTimer] = useState<ReturnType<typeof setTimeout> | null>(null);
  const [audioLevels, setAudioLevels] = useState<number[]>(new Array(24).fill(0));
  const levelsRef = useRef(audioLevels);

  // Sync recording state to overlay mode
  useEffect(() => {
    if (recordingState === "recording") {
      setMode("recording");
      if (rewriteTimer) clearTimeout(rewriteTimer);
    } else if (recordingState === "transcribing" && mode !== "rewriting") {
      setMode("transcribing");
    }
    // Reset levels when not recording
    if (recordingState !== "recording") {
      const zeros = new Array(24).fill(0);
      setAudioLevels(zeros);
      levelsRef.current = zeros;
    }
  }, [recordingState]);

  // Listen for audio level updates
  useEffect(() => {
    const unlisten = listen<number[]>("audio-levels", (event) => {
      levelsRef.current = event.payload;
      setAudioLevels(event.payload);
    });
    return () => { unlisten.then((fn) => fn()); };
  }, []);

  // Listen for transcription complete — show rewrite prompt briefly
  useEffect(() => {
    const unlistenComplete = listen<{ text: string; segment_count: number }>("transcription-complete", (event) => {
      setSegmentCount(event.payload.segment_count);
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

    const unlistenCleared = listen("composition-cleared", () => {
      setSegmentCount(0);
    });

    return () => {
      unlistenComplete.then((fn) => fn());
      unlistenRewriteStart.then((fn) => fn());
      unlistenRewriteDone.then((fn) => fn());
      unlistenRewriteError.then((fn) => fn());
      unlistenCleared.then((fn) => fn());
    };
  }, []);

  const handleRewrite = async () => {
    if (rewriteTimer) clearTimeout(rewriteTimer);
    setMode("rewriting");
    try {
      await rewriteLastTranscription();
      setMode("rewrite-done");
      setTimeout(() => setMode("idle"), 1500);
      setSegmentCount(0);
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
            <Waveform levels={audioLevels} />
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
            {segmentCount > 1 && (
              <span className="text-xs font-medium text-white/70">{segmentCount} segments</span>
            )}
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
