import { useEffect, useState } from "react";
import { useSettings } from "../hooks/useSettings";
import { getDictionary, updateDictionary } from "../lib/commands";
import type { DictionaryEntry } from "../lib/types";
import History from "./History";

type Tab = "general" | "audio" | "dictionary" | "history" | "about";

/** Main settings window with tabbed navigation. */
export default function Settings() {
  const { settings, loading, error, save } = useSettings();
  const [activeTab, setActiveTab] = useState<Tab>("general");

  if (loading) {
    return (
      <div className="settings-window flex items-center justify-center h-screen">
        <p className="text-gray-400">Loading settings...</p>
      </div>
    );
  }

  if (error || !settings) {
    return (
      <div className="settings-window flex items-center justify-center h-screen">
        <p className="text-red-400">Error: {error}</p>
      </div>
    );
  }

  const tabs: { id: Tab; label: string }[] = [
    { id: "general", label: "General" },
    { id: "audio", label: "Audio" },
    { id: "dictionary", label: "Dictionary" },
    { id: "history", label: "History" },
    { id: "about", label: "About" },
  ];

  return (
    <div className="settings-window min-h-screen p-6">
      <h1 className="text-xl font-bold mb-6">voice2txt Settings</h1>

      {/* Tab bar */}
      <div className="flex gap-1 mb-6 border-b border-gray-700">
        {tabs.map((tab) => (
          <button
            key={tab.id}
            onClick={() => setActiveTab(tab.id)}
            className={`px-4 py-2 text-sm font-medium rounded-t transition-colors ${
              activeTab === tab.id
                ? "bg-gray-700 text-white"
                : "text-gray-400 hover:text-white"
            }`}
          >
            {tab.label}
          </button>
        ))}
      </div>

      {/* Tab content */}
      <div className="max-w-lg">
        {activeTab === "general" && (
          <div className="space-y-4">
            <div>
              <label className="block text-sm text-gray-400 mb-1">
                Whisper Model
              </label>
              <select
                value={settings.model_size}
                onChange={(e) =>
                  save({ ...settings, model_size: e.target.value as any })
                }
                className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-white"
              >
                <option value="Tiny">Tiny (75MB)</option>
                <option value="Base">Base (142MB)</option>
                <option value="Small">Small (466MB)</option>
                <option value="Medium">Medium (1.5GB)</option>
                <option value="LargeV3">Large V3 (3.1GB)</option>
                <option value="LargeV3Turbo">Large V3 Turbo (1.6GB)</option>
              </select>
            </div>
            <div>
              <label className="block text-sm text-gray-400 mb-1">
                Hotkey
              </label>
              <input
                type="text"
                value={settings.shortcut}
                readOnly
                className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-white"
              />
            </div>
            <div>
              <label className="block text-sm text-gray-400 mb-1">
                Mode
              </label>
              <select
                value={settings.hotkey_mode}
                onChange={(e) =>
                  save({ ...settings, hotkey_mode: e.target.value as any })
                }
                className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-white"
              >
                <option value="PushToTalk">Push to Talk</option>
                <option value="Toggle">Toggle</option>
              </select>
            </div>
            <div>
              <label className="block text-sm text-gray-400 mb-1">
                Injection Method
              </label>
              <select
                value={settings.injection_method}
                onChange={(e) =>
                  save({
                    ...settings,
                    injection_method: e.target.value as any,
                  })
                }
                className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-white"
              >
                <option value="Clipboard">Clipboard (Cmd+V)</option>
                <option value="Keyboard">Keyboard Simulation</option>
              </select>
            </div>
          </div>
        )}

        {activeTab === "audio" && (
          <div className="space-y-5">
            <div>
              <label className="block text-sm text-gray-400 mb-2">
                VAD Silence Threshold: {settings.vad_threshold.toFixed(3)}
              </label>
              <input
                type="range"
                min="0.001"
                max="0.1"
                step="0.001"
                value={settings.vad_threshold}
                onChange={(e) =>
                  save({
                    ...settings,
                    vad_threshold: parseFloat(e.target.value),
                  })
                }
                className="w-full accent-blue-500"
              />
              <p className="text-gray-500 text-xs mt-1">
                Lower = more sensitive, higher = ignores quiet sounds
              </p>
            </div>
            <label className="flex items-center gap-3 cursor-pointer">
              <input
                type="checkbox"
                checked={settings.show_overlay}
                onChange={(e) =>
                  save({ ...settings, show_overlay: e.target.checked })
                }
                className="w-4 h-4 accent-blue-500"
              />
              <span className="text-sm text-gray-300">
                Show floating overlay during recording
              </span>
            </label>
            <label className="flex items-center gap-3 cursor-pointer">
              <input
                type="checkbox"
                checked={settings.sound_feedback}
                onChange={(e) =>
                  save({ ...settings, sound_feedback: e.target.checked })
                }
                className="w-4 h-4 accent-blue-500"
              />
              <span className="text-sm text-gray-300">
                Play sound on start/stop
              </span>
            </label>
          </div>
        )}

        {activeTab === "dictionary" && <DictionaryEditor />}

        {activeTab === "history" && <History />}

        {activeTab === "about" && (
          <div className="space-y-2">
            <p className="text-white font-medium">voice2txt v0.1.0</p>
            <p className="text-gray-400 text-sm">
              Voice-to-text developer tool for macOS.
            </p>
            <p className="text-gray-400 text-sm">
              Built with Tauri + whisper.cpp + React.
            </p>
          </div>
        )}
      </div>
    </div>
  );
}

/** Inline dictionary editor with CRUD operations. */
function DictionaryEditor() {
  const [entries, setEntries] = useState<DictionaryEntry[]>([]);
  const [loading, setLoading] = useState(true);
  const [saved, setSaved] = useState(false);

  useEffect(() => {
    getDictionary()
      .then(setEntries)
      .finally(() => setLoading(false));
  }, []);

  const handleSave = async () => {
    const filtered = entries.filter((e) => e.from.trim() !== "");
    await updateDictionary(filtered);
    setEntries(filtered);
    setSaved(true);
    setTimeout(() => setSaved(false), 2000);
  };

  const addRow = () => setEntries([...entries, { from: "", to: "" }]);

  const removeRow = (i: number) =>
    setEntries(entries.filter((_, idx) => idx !== i));

  const updateRow = (i: number, field: "from" | "to", value: string) =>
    setEntries(
      entries.map((e, idx) => (idx === i ? { ...e, [field]: value } : e))
    );

  if (loading) {
    return <p className="text-gray-400 text-sm">Loading dictionary...</p>;
  }

  return (
    <div className="space-y-4">
      <p className="text-gray-400 text-xs">
        Add corrections for terms Whisper often gets wrong (e.g., "react
        native" &rarr; "React Native").
      </p>

      {entries.length > 0 && (
        <table className="w-full text-sm">
          <thead>
            <tr className="text-gray-400 text-left">
              <th className="pb-2">Whisper Output</th>
              <th className="pb-2">Corrected To</th>
              <th className="pb-2 w-16"></th>
            </tr>
          </thead>
          <tbody>
            {entries.map((entry, i) => (
              <tr key={i}>
                <td className="pr-2 pb-2">
                  <input
                    value={entry.from}
                    onChange={(e) => updateRow(i, "from", e.target.value)}
                    placeholder="whisper output"
                    className="w-full bg-gray-800 border border-gray-600 rounded px-2 py-1 text-white placeholder-gray-600"
                  />
                </td>
                <td className="pr-2 pb-2">
                  <input
                    value={entry.to}
                    onChange={(e) => updateRow(i, "to", e.target.value)}
                    placeholder="correction"
                    className="w-full bg-gray-800 border border-gray-600 rounded px-2 py-1 text-white placeholder-gray-600"
                  />
                </td>
                <td className="pb-2 text-center">
                  <button
                    onClick={() => removeRow(i)}
                    className="text-red-400 hover:text-red-300 text-xs"
                  >
                    Remove
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      )}

      <div className="flex gap-2 items-center">
        <button
          onClick={addRow}
          className="bg-gray-700 hover:bg-gray-600 text-white px-3 py-1 rounded text-sm"
        >
          + Add Entry
        </button>
        <button
          onClick={handleSave}
          className="bg-blue-600 hover:bg-blue-700 text-white px-3 py-1 rounded text-sm"
        >
          Save Dictionary
        </button>
        {saved && (
          <span className="text-green-400 text-xs">Saved!</span>
        )}
      </div>
    </div>
  );
}
