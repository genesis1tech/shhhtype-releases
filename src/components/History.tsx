import { useState } from "react";
import { useHistory } from "../hooks/useHistory";
import { exportHistory } from "../lib/commands";

/** Searchable transcription history list with export. */
export default function History() {
  const [search, setSearch] = useState("");
  const { entries, loading, error, fetch, remove } = useHistory();
  const [exporting, setExporting] = useState(false);

  const handleSearch = () => {
    fetch({ search: search || undefined, limit: 50 });
  };

  const handleExport = async () => {
    setExporting(true);
    try {
      const allEntries = await exportHistory();
      const json = JSON.stringify(allEntries, null, 2);
      const blob = new Blob([json], { type: "application/json" });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = `shhhtype-history-${new Date().toISOString().slice(0, 10)}.json`;
      a.click();
      URL.revokeObjectURL(url);
    } catch (e) {
      console.error("Export failed:", e);
    } finally {
      setExporting(false);
    }
  };

  return (
    <div className="space-y-4">
      <div className="flex gap-2">
        <input
          type="text"
          placeholder="Search transcriptions..."
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          onKeyDown={(e) => e.key === "Enter" && handleSearch()}
          className="apple-input flex-1"
        />
        <button onClick={handleSearch} className="apple-button">
          Search
        </button>
        <button
          onClick={handleExport}
          disabled={exporting}
          className="apple-button-secondary"
        >
          {exporting ? "Exporting..." : "Export"}
        </button>
      </div>

      {loading && <p className="text-white/55 text-[13px]">Loading...</p>}
      {error && <p className="text-[#FF3B30] text-[13px]">Error: {error}</p>}

      {entries.length > 0 && (
        <div className="settings-group">
          {entries.map((entry, i) => (
            <div
              key={entry.id}
              className={`flex justify-between items-start px-4 py-3${
                i > 0 ? " border-t border-white/10" : ""
              }`}
            >
              <div className="flex-1 min-w-0">
                <p className="text-white/85 text-[13px] truncate">{entry.text}</p>
                <p className="text-white/30 text-[11px] mt-1">
                  {new Date(entry.created_at).toLocaleString()} &middot;{" "}
                  {entry.word_count} words &middot;{" "}
                  {(entry.duration_ms / 1000).toFixed(1)}s
                </p>
              </div>
              <button
                onClick={() => remove(entry.id)}
                className="text-white/30 hover:text-[#FF3B30] text-[11px] ml-2 shrink-0"
              >
                Delete
              </button>
            </div>
          ))}
        </div>
      )}
      {!loading && entries.length === 0 && (
        <p className="text-white/30 text-[13px] text-center py-8">
          No transcriptions yet.
        </p>
      )}
    </div>
  );
}
