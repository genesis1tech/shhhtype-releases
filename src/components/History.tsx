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
          className="flex-1 bg-gray-800 border border-gray-600 rounded px-3 py-2 text-white text-sm"
        />
        <button
          onClick={handleSearch}
          className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded text-sm"
        >
          Search
        </button>
        <button
          onClick={handleExport}
          disabled={exporting}
          className="bg-gray-700 hover:bg-gray-600 text-white px-3 py-2 rounded text-sm disabled:opacity-50"
        >
          {exporting ? "Exporting..." : "Export"}
        </button>
      </div>

      {loading && <p className="text-gray-400 text-sm">Loading...</p>}
      {error && <p className="text-red-400 text-sm">Error: {error}</p>}

      <div className="space-y-2">
        {entries.map((entry) => (
          <div
            key={entry.id}
            className="bg-gray-800 rounded p-3 flex justify-between items-start"
          >
            <div className="flex-1 min-w-0">
              <p className="text-white text-sm truncate">{entry.text}</p>
              <p className="text-gray-500 text-xs mt-1">
                {new Date(entry.created_at).toLocaleString()} &middot;{" "}
                {entry.word_count} words &middot;{" "}
                {(entry.duration_ms / 1000).toFixed(1)}s
              </p>
            </div>
            <button
              onClick={() => remove(entry.id)}
              className="text-gray-500 hover:text-red-400 text-xs ml-2"
            >
              Delete
            </button>
          </div>
        ))}
        {!loading && entries.length === 0 && (
          <p className="text-gray-500 text-sm text-center py-8">
            No transcriptions yet.
          </p>
        )}
      </div>
    </div>
  );
}
