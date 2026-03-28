import { useRecordingState } from "../hooks/useTauriEvents";

/** Floating transparent overlay showing recording status. */
export default function Overlay() {
  const state = useRecordingState();

  if (state === "idle") {
    return <div className="overlay-window h-screen" />;
  }

  return (
    <div className="overlay-window flex items-center justify-center h-screen">
      <div className="flex items-center gap-3 rounded-full bg-black/80 px-5 py-3 shadow-2xl backdrop-blur-sm">
        <span
          className={`inline-block h-3 w-3 rounded-full ${
            state === "recording"
              ? "bg-red-500 animate-pulse"
              : "bg-yellow-400 animate-pulse"
          }`}
        />
        <span className="text-sm font-medium text-white">
          {state === "recording" ? "Listening..." : "Transcribing..."}
        </span>
      </div>
    </div>
  );
}
