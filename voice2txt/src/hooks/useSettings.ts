import { useEffect, useState, useCallback } from "react";
import type { Settings } from "../lib/types";
import { getSettings, updateSettings } from "../lib/commands";

/** Hook for reading and updating application settings. */
export function useSettings() {
  const [settings, setSettings] = useState<Settings | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    getSettings()
      .then((s) => {
        setSettings(s);
        setLoading(false);
      })
      .catch((e) => {
        setError(String(e));
        setLoading(false);
      });
  }, []);

  const save = useCallback(
    async (updated: Settings) => {
      try {
        await updateSettings(updated);
        setSettings(updated);
        setError(null);
      } catch (e) {
        setError(String(e));
      }
    },
    [],
  );

  return { settings, loading, error, save };
}
