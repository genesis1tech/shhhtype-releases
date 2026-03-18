import { useEffect, useState, useRef, useCallback } from "react";
import { listen } from "@tauri-apps/api/event";
import { useSettings } from "../hooks/useSettings";
import {
  getDictionary,
  updateDictionary,
  getModelStatus,
  downloadModel,
  deleteModel,
  checkPermissions,
  requestMicrophonePermission,
  activateLicense,
  getLicenseStatus,
  deactivateLicense,
  getGroqUsage,
} from "../lib/commands";
import type {
  DictionaryEntry,
  ModelStatus,
  ModelSize,
  DownloadProgress,
  PermissionStatus as PermStatus,
  TranscriptionBackend,
  RewriteStyle,
  LicenseStatus,
  GroqUsage,
} from "../lib/types";
import History from "./History";

type Tab = "general" | "audio" | "dictionary" | "history" | "license" | "about";

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
    { id: "license", label: "License" },
    { id: "about", label: "About" },
  ];

  return (
    <div className="settings-window min-h-screen p-6">
      <h1 className="text-xl font-bold mb-6">ShhhType Settings</h1>

      {/* Permission status banner */}
      <PermissionBanner />

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
          <GeneralTab settings={settings} save={save} />
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
            <div>
              <label className="block text-sm text-gray-400 mb-2">
                Silence Timeout: {settings.vad_silence_timeout.toFixed(0)}s
              </label>
              <input
                type="range"
                min="1"
                max="30"
                step="1"
                value={settings.vad_silence_timeout}
                onChange={(e) =>
                  save({
                    ...settings,
                    vad_silence_timeout: parseFloat(e.target.value),
                  })
                }
                className="w-full accent-blue-500"
              />
              <p className="text-gray-500 text-xs mt-1">
                How long to keep listening after you stop speaking before auto-stopping
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

        {activeTab === "license" && <LicenseTab />}

        {activeTab === "about" && (
          <div className="space-y-3">
            <p className="text-white font-medium">ShhhType v0.1.0</p>
            <p className="text-gray-400 text-sm">
              Voice-to-text developer tool for macOS.
            </p>
            <p className="text-gray-400 text-sm">
              Built with Tauri + whisper.cpp + React.
            </p>
            <div className="pt-2 space-y-1">
              <a
                href="https://github.com/genesis1tech/shhhtype"
                target="_blank"
                rel="noopener noreferrer"
                className="block text-blue-400 hover:underline text-sm"
              >
                GitHub Repository
              </a>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

/** Permission status banner - only shows when permissions are missing. */
function PermissionBanner() {
  const [status, setStatus] = useState<PermStatus | null>(null);
  const [requesting, setRequesting] = useState(false);

  const refreshStatus = () => {
    checkPermissions().then(setStatus).catch(console.error);
  };

  useEffect(() => {
    refreshStatus();
    // Poll permissions every 2s to detect when user grants them in System Settings
    const interval = setInterval(refreshStatus, 2000);
    return () => clearInterval(interval);
  }, []);

  const handleRequestMic = async () => {
    setRequesting(true);
    try {
      await requestMicrophonePermission();
      // Wait a moment for macOS to process, then refresh
      setTimeout(refreshStatus, 1000);
    } catch (e) {
      console.error("Failed to request microphone permission:", e);
    } finally {
      setRequesting(false);
    }
  };

  if (!status) return null;
  if (status.microphone && status.accessibility) return null;

  return (
    <div className="bg-yellow-900/50 border border-yellow-700 rounded p-4 mb-6 space-y-2">
      <h3 className="text-yellow-300 font-medium text-sm">
        Permissions Required
      </h3>
      <div className="space-y-1 text-sm">
        <div className="flex items-center gap-2">
          <span
            className={`inline-block h-2 w-2 rounded-full ${
              status.microphone ? "bg-green-400" : "bg-red-400"
            }`}
          />
          <span
            className={
              status.microphone ? "text-green-300" : "text-red-300"
            }
          >
            Microphone: {status.microphone ? "Granted" : "Not Granted"}
          </span>
          {!status.microphone && (
            <button
              onClick={handleRequestMic}
              disabled={requesting}
              className="ml-2 bg-blue-600 hover:bg-blue-700 disabled:opacity-50 text-white px-2 py-0.5 rounded text-xs"
            >
              {requesting ? "Requesting..." : "Grant Access"}
            </button>
          )}
        </div>
        <div className="flex items-center gap-2">
          <span
            className={`inline-block h-2 w-2 rounded-full ${
              status.accessibility ? "bg-green-400" : "bg-red-400"
            }`}
          />
          <span
            className={
              status.accessibility ? "text-green-300" : "text-red-300"
            }
          >
            Accessibility:{" "}
            {status.accessibility ? "Granted" : "Not Granted"}
          </span>
        </div>
      </div>
      <p className="text-gray-400 text-xs">
        Click "Grant Access" to trigger the permission prompt, or open System Settings &gt; Privacy &amp; Security manually.
      </p>
    </div>
  );
}

/** General tab with model management, hotkey, injection, and launch settings. */
function GeneralTab({
  settings,
  save,
}: {
  settings: NonNullable<ReturnType<typeof useSettings>["settings"]>;
  save: ReturnType<typeof useSettings>["save"];
}) {
  const [models, setModels] = useState<ModelStatus[]>([]);
  const [downloading, setDownloading] = useState<string | null>(null);
  const [progress, setProgress] = useState(0);
  const [downloadError, setDownloadError] = useState<string | null>(null);
  const [capturingHotkey, setCapturingHotkey] = useState(false);
  const hotkeyRef = useRef<HTMLInputElement>(null);

  const loadModels = () => {
    getModelStatus().then(setModels).catch(console.error);
  };

  useEffect(() => {
    loadModels();
  }, []);

  useEffect(() => {
    const unlisten = listen<DownloadProgress>(
      "model-download-progress",
      (event) => {
        setProgress(Math.round(event.payload.percent));
      }
    );
    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  const currentModel = models.find((m) => m.model === settings.model_size);
  const isCurrentDownloaded = currentModel?.downloaded ?? false;

  const handleDownload = async () => {
    setDownloading(settings.model_size);
    setProgress(0);
    setDownloadError(null);
    try {
      await downloadModel(settings.model_size);
      loadModels();
    } catch (e) {
      setDownloadError(String(e));
    } finally {
      setDownloading(null);
    }
  };

  const handleDeleteModel = async (modelId: ModelSize) => {
    try {
      await deleteModel(modelId);
      loadModels();
    } catch (e) {
      console.error("Failed to delete model:", e);
    }
  };

  // Hotkey capture: convert keyboard event to Tauri shortcut string
  const handleHotkeyCapture = useCallback(
    (e: React.KeyboardEvent) => {
      e.preventDefault();
      e.stopPropagation();

      const parts: string[] = [];
      if (e.metaKey || e.ctrlKey) parts.push("CmdOrCtrl");
      if (e.shiftKey) parts.push("Shift");
      if (e.altKey) parts.push("Alt");

      const key = e.key;
      // Only accept if a modifier + a non-modifier key is pressed
      if (
        key !== "Meta" &&
        key !== "Control" &&
        key !== "Shift" &&
        key !== "Alt"
      ) {
        // Map common keys to Tauri shortcut names
        const keyMap: Record<string, string> = {
          " ": "Space",
          ArrowUp: "Up",
          ArrowDown: "Down",
          ArrowLeft: "Left",
          ArrowRight: "Right",
          Enter: "Return",
          Backspace: "Backspace",
          Delete: "Delete",
          Escape: "Escape",
          Tab: "Tab",
        };
        const mappedKey =
          keyMap[key] || (key.length === 1 ? key.toUpperCase() : key);
        parts.push(mappedKey);

        if (parts.length >= 2) {
          const shortcut = parts.join("+");
          save({ ...settings, shortcut });
          setCapturingHotkey(false);
        }
      }
    },
    [settings, save]
  );

  const modelOptions: { value: ModelSize; label: string }[] = [
    { value: "Tiny", label: "Tiny (75MB)" },
    { value: "Base", label: "Base (142MB)" },
    { value: "Small", label: "Small (466MB)" },
    { value: "Medium", label: "Medium (1.5GB)" },
    { value: "LargeV3", label: "Large V3 (3.1GB)" },
    { value: "LargeV3Turbo", label: "Large V3 Turbo (1.6GB)" },
  ];

  return (
    <div className="space-y-4">
      {/* Model selector — only for local backend */}
      {settings.transcription_backend === "Local" && <div>
        <label className="block text-sm text-gray-400 mb-1">
          Whisper Model
        </label>
        <select
          value={settings.model_size}
          onChange={(e) =>
            save({ ...settings, model_size: e.target.value as ModelSize })
          }
          className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-white"
        >
          {modelOptions.map((opt) => {
            const status = models.find((m) => m.model === opt.value);
            const dot = status?.downloaded ? "\u2713 " : "\u25CB ";
            return (
              <option key={opt.value} value={opt.value}>
                {dot}
                {opt.label}
              </option>
            );
          })}
        </select>

        {/* Download / status */}
        {models.length > 0 && !isCurrentDownloaded && !downloading && (
          <div className="mt-2">
            <button
              onClick={handleDownload}
              className="bg-blue-600 hover:bg-blue-700 text-white px-3 py-1.5 rounded text-sm"
            >
              Download {settings.model_size} model
            </button>
            <p className="text-gray-500 text-xs mt-1">
              Model will be downloaded from Hugging Face.
            </p>
          </div>
        )}

        {downloading && (
          <div className="mt-2 space-y-1">
            <div className="flex items-center gap-2">
              <div className="flex-1 bg-gray-700 rounded-full h-2">
                <div
                  className="bg-blue-500 h-2 rounded-full transition-all"
                  style={{ width: `${progress}%` }}
                />
              </div>
              <span className="text-xs text-gray-400 w-10 text-right">
                {progress}%
              </span>
            </div>
            <p className="text-gray-500 text-xs">
              Downloading {downloading} model...
            </p>
          </div>
        )}

        {downloadError && (
          <p className="text-red-400 text-xs mt-2">
            Download failed: {downloadError}
          </p>
        )}

        {models.length > 0 && isCurrentDownloaded && !downloading && (
          <div className="flex items-center gap-2 mt-1">
            <p className="text-green-400 text-xs">Model ready</p>
            <button
              onClick={() => handleDeleteModel(settings.model_size)}
              className="text-gray-500 hover:text-red-400 text-xs"
            >
              Delete
            </button>
          </div>
        )}
      </div>}

      {/* Language */}
      <div>
        <label className="block text-sm text-gray-400 mb-1">Language</label>
        <select
          value={settings.language}
          onChange={(e) => save({ ...settings, language: e.target.value })}
          className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-white"
        >
          <option value="en">English</option>
          <option value="es">Spanish</option>
          <option value="fr">French</option>
          <option value="de">German</option>
          <option value="it">Italian</option>
          <option value="pt">Portuguese</option>
          <option value="ja">Japanese</option>
          <option value="ko">Korean</option>
          <option value="zh">Chinese</option>
          <option value="auto">Auto-detect</option>
        </select>
      </div>

      {/* Hotkey */}
      <div>
        <label className="block text-sm text-gray-400 mb-1">Hotkey</label>
        <div className="flex gap-2">
          <input
            ref={hotkeyRef}
            type="text"
            value={
              capturingHotkey
                ? "Press shortcut..."
                : settings.shortcut
            }
            readOnly={!capturingHotkey}
            onKeyDown={capturingHotkey ? handleHotkeyCapture : undefined}
            onBlur={() => setCapturingHotkey(false)}
            className={`flex-1 bg-gray-800 border rounded px-3 py-2 text-white ${
              capturingHotkey
                ? "border-blue-500 ring-1 ring-blue-500"
                : "border-gray-600"
            }`}
          />
          <button
            onClick={() => {
              setCapturingHotkey(true);
              setTimeout(() => hotkeyRef.current?.focus(), 50);
            }}
            className="bg-gray-700 hover:bg-gray-600 text-white px-3 py-2 rounded text-sm"
          >
            {capturingHotkey ? "Cancel" : "Change"}
          </button>
        </div>
        <p className="text-gray-500 text-xs mt-1">
          {capturingHotkey
            ? "Press a modifier + key combination (e.g., Cmd+Shift+Space)"
            : "Click Change to set a new hotkey. Requires app restart to take effect."}
        </p>
      </div>

      {/* Mode */}
      <div>
        <label className="block text-sm text-gray-400 mb-1">Mode</label>
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

      {/* Injection Method */}
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

      {/* Auto-copy toggle */}
      <label className="flex items-center gap-3 cursor-pointer">
        <input
          type="checkbox"
          checked={settings.auto_copy}
          onChange={(e) =>
            save({ ...settings, auto_copy: e.target.checked })
          }
          className="w-4 h-4 accent-blue-500"
        />
        <div>
          <span className="text-sm text-gray-300">
            Auto-copy transcription to clipboard
          </span>
          <p className="text-gray-500 text-xs">
            When using keyboard injection, also copy text to clipboard
          </p>
        </div>
      </label>

      {/* Auto-launch toggle */}
      <label className="flex items-center gap-3 cursor-pointer">
        <input
          type="checkbox"
          checked={settings.auto_launch}
          onChange={(e) =>
            save({ ...settings, auto_launch: e.target.checked })
          }
          className="w-4 h-4 accent-blue-500"
        />
        <span className="text-sm text-gray-300">Launch at login</span>
      </label>

      {/* Divider */}
      <hr className="border-gray-700" />

      {/* Transcription Backend */}
      <div>
        <label className="block text-sm text-gray-400 mb-1">
          Transcription Backend
        </label>
        <select
          value={settings.transcription_backend}
          onChange={(e) =>
            save({
              ...settings,
              transcription_backend: e.target.value as TranscriptionBackend,
            })
          }
          className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-white"
        >
          <option value="Cloud">Cloud (Groq)</option>
          <option value="Local">Local (whisper.cpp)</option>
        </select>
        <p className="text-gray-500 text-xs mt-1">
          {settings.transcription_backend === "Cloud"
            ? "Fast cloud transcription via Groq API. Requires API key."
            : "Private, on-device transcription. No data leaves your Mac."}
        </p>
      </div>

      {/* Groq API Key — shown when Cloud backend or rewrite is enabled */}
      {(settings.transcription_backend === "Cloud" || settings.rewrite_enabled) && (
        <div>
          <label className="block text-sm text-gray-400 mb-1">
            Groq API Key
          </label>
          <input
            type="password"
            value={settings.groq_api_key ?? ""}
            onChange={(e) =>
              save({
                ...settings,
                groq_api_key: e.target.value || null,
              })
            }
            placeholder="gsk_..."
            className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-white placeholder-gray-600"
          />
          <p className="text-gray-500 text-xs mt-1">
            Get a free API key at{" "}
            <a
              href="https://console.groq.com"
              target="_blank"
              rel="noopener noreferrer"
              className="text-blue-400 hover:underline"
            >
              console.groq.com
            </a>
          </p>
        </div>
      )}

      {/* Groq Usage — shown when Cloud backend or rewrite is enabled */}
      {(settings.transcription_backend === "Cloud" || settings.rewrite_enabled) && (
        <GroqUsageCard />
      )}

      {/* Divider */}
      <hr className="border-gray-700" />

      {/* AI Rewrite */}
      <label className="flex items-center gap-3 cursor-pointer">
        <input
          type="checkbox"
          checked={settings.rewrite_enabled}
          onChange={(e) =>
            save({ ...settings, rewrite_enabled: e.target.checked })
          }
          className="w-4 h-4 accent-blue-500"
        />
        <div>
          <span className="text-sm text-gray-300">
            AI Rewrite
          </span>
          <p className="text-gray-500 text-xs">
            After transcription, press {settings.rewrite_hotkey} to polish text with AI
          </p>
        </div>
      </label>

      {settings.rewrite_enabled && (
        <div>
          <label className="block text-sm text-gray-400 mb-1">
            Rewrite Style
          </label>
          <select
            value={settings.rewrite_style}
            onChange={(e) =>
              save({
                ...settings,
                rewrite_style: e.target.value as RewriteStyle,
              })
            }
            className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-white"
          >
            <option value="Professional">Professional</option>
            <option value="Casual">Casual</option>
            <option value="Concise">Concise</option>
            <option value="Friendly">Friendly</option>
          </select>
        </div>
      )}
    </div>
  );
}

/** Groq API usage / rate limit display card. */
function GroqUsageCard() {
  const [usage, setUsage] = useState<GroqUsage | null>(null);

  const refresh = useCallback(() => {
    getGroqUsage().then(setUsage).catch(console.error);
  }, []);

  useEffect(() => {
    refresh();
    // Refresh when transcription/rewrite completes
    const unsubs = [
      listen("transcription-complete", refresh),
      listen("rewrite-complete", refresh),
    ];
    return () => { unsubs.forEach((p) => p.then((fn) => fn())); };
  }, [refresh]);

  if (!usage || !usage.updated_at) {
    return (
      <div className="bg-gray-800/50 border border-gray-700 rounded p-3">
        <p className="text-gray-500 text-xs">
          Groq usage data will appear after your first API call.
        </p>
      </div>
    );
  }

  const reqPct =
    usage.limit_requests && usage.remaining_requests != null
      ? (usage.remaining_requests / usage.limit_requests) * 100
      : null;
  const tokPct =
    usage.limit_tokens && usage.remaining_tokens != null
      ? (usage.remaining_tokens / usage.limit_tokens) * 100
      : null;

  const barColor = (pct: number) =>
    pct > 30 ? "bg-green-500" : pct > 10 ? "bg-yellow-500" : "bg-red-500";

  return (
    <div className="bg-gray-800/50 border border-gray-700 rounded p-3 space-y-2">
      <div className="flex items-center justify-between">
        <span className="text-xs font-medium text-gray-300">Groq API Daily Usage</span>
        <button
          onClick={refresh}
          className="text-gray-500 hover:text-gray-300 text-xs"
        >
          Refresh
        </button>
      </div>

      {reqPct != null && (
        <div>
          <div className="flex justify-between text-xs text-gray-400 mb-0.5">
            <span>Requests</span>
            <span>
              {usage.remaining_requests} / {usage.limit_requests}
              {usage.reset_requests && (
                <span className="text-gray-500 ml-1">
                  (resets in {usage.reset_requests})
                </span>
              )}
            </span>
          </div>
          <div className="w-full bg-gray-700 rounded-full h-1.5">
            <div
              className={`h-1.5 rounded-full transition-all ${barColor(reqPct)}`}
              style={{ width: `${reqPct}%` }}
            />
          </div>
        </div>
      )}

      {tokPct != null && (
        <div>
          <div className="flex justify-between text-xs text-gray-400 mb-0.5">
            <span>Tokens</span>
            <span>
              {usage.remaining_tokens?.toLocaleString()} / {usage.limit_tokens?.toLocaleString()}
              {usage.reset_tokens && (
                <span className="text-gray-500 ml-1">
                  (resets in {usage.reset_tokens})
                </span>
              )}
            </span>
          </div>
          <div className="w-full bg-gray-700 rounded-full h-1.5">
            <div
              className={`h-1.5 rounded-full transition-all ${barColor(tokPct)}`}
              style={{ width: `${tokPct}%` }}
            />
          </div>
        </div>
      )}

      {usage.updated_at && (
        <p className="text-gray-600 text-[10px]">
          Last updated: {new Date(usage.updated_at).toLocaleTimeString()}
        </p>
      )}
    </div>
  );
}

/** License activation and management. */
function LicenseTab() {
  const [status, setStatus] = useState<LicenseStatus>("Free");
  const [key, setKey] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    getLicenseStatus().then(setStatus).catch(console.error);
  }, []);

  const handleActivate = async () => {
    if (!key.trim()) return;
    setLoading(true);
    setError(null);
    try {
      const result = await activateLicense(key.trim());
      setStatus(result);
      setKey("");
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  };

  const handleDeactivate = async () => {
    setLoading(true);
    setError(null);
    try {
      await deactivateLicense();
      setStatus("Free");
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-3">
        <span
          className={`inline-block h-3 w-3 rounded-full ${
            status === "Licensed"
              ? "bg-green-400"
              : status === "Invalid"
              ? "bg-red-400"
              : "bg-gray-400"
          }`}
        />
        <span className="text-sm font-medium text-white">
          {status === "Licensed"
            ? "Licensed"
            : status === "Invalid"
            ? "Invalid License"
            : "Free Version"}
        </span>
      </div>

      {status === "Licensed" ? (
        <div className="space-y-3">
          <p className="text-gray-400 text-sm">
            Your license is active on this machine. Cloud transcription and AI
            rewrite are unlocked.
          </p>
          <button
            onClick={handleDeactivate}
            disabled={loading}
            className="bg-gray-700 hover:bg-gray-600 disabled:opacity-50 text-white px-3 py-1.5 rounded text-sm"
          >
            {loading ? "Deactivating..." : "Deactivate License"}
          </button>
          <p className="text-gray-500 text-xs">
            Deactivate to transfer your license to another machine.
          </p>
        </div>
      ) : (
        <div className="space-y-3">
          <p className="text-gray-400 text-sm">
            Enter your license key to unlock cloud transcription and AI rewrite.
          </p>
          <div className="flex gap-2">
            <input
              type="text"
              value={key}
              onChange={(e) => setKey(e.target.value)}
              placeholder="Enter license key..."
              className="flex-1 bg-gray-800 border border-gray-600 rounded px-3 py-2 text-white placeholder-gray-600"
            />
            <button
              onClick={handleActivate}
              disabled={loading || !key.trim()}
              className="bg-blue-600 hover:bg-blue-700 disabled:opacity-50 text-white px-4 py-2 rounded text-sm"
            >
              {loading ? "Activating..." : "Activate"}
            </button>
          </div>
          {error && <p className="text-red-400 text-xs">{error}</p>}
        </div>
      )}
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
