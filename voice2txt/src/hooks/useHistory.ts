import { useEffect, useState, useCallback } from "react";
import type { HistoryEntry, HistoryQuery } from "../lib/types";
import { getHistory, deleteHistoryEntry } from "../lib/commands";

/** Hook for querying and managing transcription history. */
export function useHistory(initialQuery?: HistoryQuery) {
  const [entries, setEntries] = useState<HistoryEntry[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetch = useCallback(async (query?: HistoryQuery) => {
    setLoading(true);
    try {
      const data = await getHistory(query ?? {});
      setEntries(data);
      setError(null);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetch(initialQuery);
  }, [fetch, initialQuery]);

  const remove = useCallback(
    async (id: string) => {
      try {
        await deleteHistoryEntry(id);
        setEntries((prev) => prev.filter((e) => e.id !== id));
      } catch (e) {
        setError(String(e));
      }
    },
    [],
  );

  return { entries, loading, error, fetch, remove };
}
