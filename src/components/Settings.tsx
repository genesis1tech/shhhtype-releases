import { useEffect, useState, useRef, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
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
  listAudioDevices,
  restartApp,
  getUpdateInfo,
  checkForUpdates,
} from "../lib/commands";
import type {
  AudioDevice,
  DictionaryEntry,
  ModelStatus,
  ModelSize,
  DownloadProgress,
  PermissionStatus as PermStatus,
  TranscriptionBackend,
  RewriteStyle,
  LicenseStatus,
  GroqUsage,
  UpdateInfo,
  OverlayPosition,
} from "../lib/types";
import History from "./History";

type Tab = "general" | "audio" | "dictionary" | "history" | "license" | "about";

const SIDEBAR_TABS: { id: Tab; label: string; color: string }[] = [
  { id: "general", label: "General", color: "#8E8E93" },
  { id: "audio", label: "Audio", color: "#AF52DE" },
  { id: "dictionary", label: "Dictionary", color: "#FF9500" },
  { id: "history", label: "History", color: "#34C759" },
  { id: "license", label: "License", color: "#FFCC00" },
  { id: "about", label: "About", color: "#007AFF" },
];

function SidebarIcon({ tab }: { tab: Tab }) {
  const svgs: Record<Tab, React.ReactNode> = {
    general: (
      <svg viewBox="0 0 16 16" fill="none" stroke="white" strokeWidth="1.5" strokeLinecap="round">
        <circle cx="8" cy="8" r="2" />
        <path d="M8 1.5v2M8 12.5v2M1.5 8h2M12.5 8h2M3.4 3.4l1.4 1.4M11.2 11.2l1.4 1.4M3.4 12.6l1.4-1.4M11.2 4.8l1.4-1.4" />
      </svg>
    ),
    audio: (
      <svg viewBox="0 0 16 16" fill="white">
        <rect x="2" y="5.5" width="2" height="5" rx="1" />
        <rect x="5.5" y="3" width="2" height="10" rx="1" />
        <rect x="9" y="4.5" width="2" height="7" rx="1" />
        <rect x="12.5" y="6" width="2" height="4" rx="1" />
      </svg>
    ),
    dictionary: (
      <svg viewBox="0 0 16 16" fill="none" stroke="white" strokeWidth="1.5" strokeLinecap="round">
        <rect x="3" y="1.5" width="10" height="13" rx="1.5" />
        <path d="M6 5h4M6 8h2.5" />
      </svg>
    ),
    history: (
      <svg viewBox="0 0 16 16" fill="none" stroke="white" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round">
        <circle cx="8" cy="8" r="6" />
        <path d="M8 4.5V8l2.5 1.5" />
      </svg>
    ),
    license: (
      <svg viewBox="0 0 16 16" fill="none" stroke="white" strokeWidth="1.5" strokeLinecap="round">
        <rect x="2" y="5" width="12" height="8" rx="1.5" />
        <path d="M5 5V3.5a3 3 0 016 0V5" />
      </svg>
    ),
    about: (
      <svg viewBox="0 0 16 16" fill="none" stroke="white" strokeWidth="1.5" strokeLinecap="round">
        <circle cx="8" cy="8" r="6" />
        <path d="M8 7v4M8 5v.01" />
      </svg>
    ),
  };
  return <>{svgs[tab]}</>;
}

function SettingsGroup({ title, children }: { title?: string; children: React.ReactNode }) {
  return (
    <div className="mb-5">
      {title && <h3 className="settings-section-header">{title}</h3>}
      <div className="settings-group">{children}</div>
    </div>
  );
}

function SettingsRow({
  label,
  description,
  children,
}: {
  label: string;
  description?: React.ReactNode;
  children: React.ReactNode;
}) {
  return (
    <div className="settings-row">
      <div className="flex-1 min-w-0">
        <div className="text-[13px] text-white/85">{label}</div>
        {description && (
          <div className="text-[11px] text-white/55 mt-0.5">{description}</div>
        )}
      </div>
      <div className="flex items-center gap-2 shrink-0">{children}</div>
    </div>
  );
}

/** Main settings window with Apple System Settings-style sidebar navigation. */
export default function Settings() {
  const { settings, loading, error, save } = useSettings();
  const [activeTab, setActiveTab] = useState<Tab>("general");

  if (loading) {
    return (
      <div className="settings-window flex items-center justify-center h-screen">
        <p className="text-white/55">Loading settings...</p>
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

  return (
    <div className="settings-layout">
      <div className="settings-sidebar">
        {SIDEBAR_TABS.map((tab) => (
          <button
            key={tab.id}
            onClick={() => setActiveTab(tab.id)}
            className={`sidebar-item ${activeTab === tab.id ? "active" : ""}`}
          >
            <span
              className="sidebar-icon-badge"
              style={{ background: tab.color }}
            >
              <SidebarIcon tab={tab.id} />
            </span>
            {tab.label}
          </button>
        ))}
      </div>
      <div className="settings-content">
        <PermissionBanner />
        {activeTab === "general" && (
          <GeneralTab settings={settings} save={save} />
        )}
        {activeTab === "audio" && (
          <AudioTab settings={settings} save={save} />
        )}
        {activeTab === "dictionary" && <DictionaryEditor />}
        {activeTab === "history" && <History />}
        {activeTab === "license" && <LicenseTab />}
        {activeTab === "about" && <AboutTab />}
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
    const interval = setInterval(refreshStatus, 2000);
    return () => clearInterval(interval);
  }, []);

  const handleRequestMic = async () => {
    setRequesting(true);
    try {
      await requestMicrophonePermission();
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
    <div className="bg-white/5 border border-white/10 rounded-[10px] p-4 mb-5">
      <h3 className="text-[#FFCC00] font-medium text-[13px] mb-2">
        Permissions Required
      </h3>
      <div className="space-y-1.5 text-[13px]">
        <div className="flex items-center gap-2">
          <span
            className={`inline-block h-2 w-2 rounded-full ${
              status.microphone ? "bg-[#34C759]" : "bg-[#FF3B30]"
            }`}
          />
          <span
            className={
              status.microphone ? "text-[#34C759]" : "text-[#FF3B30]"
            }
          >
            Microphone: {status.microphone ? "Granted" : "Not Granted"}
          </span>
          {!status.microphone && (
            <button
              onClick={handleRequestMic}
              disabled={requesting}
              className="bg-[#007AFF] text-white text-[11px] px-2 py-0.5 rounded hover:opacity-85 disabled:opacity-50 ml-2"
            >
              {requesting ? "Requesting..." : "Grant Access"}
            </button>
          )}
        </div>
        <div className="flex items-center gap-2">
          <span
            className={`inline-block h-2 w-2 rounded-full ${
              status.accessibility ? "bg-[#34C759]" : "bg-[#FF3B30]"
            }`}
          />
          <span
            className={
              status.accessibility ? "text-[#34C759]" : "text-[#FF3B30]"
            }
          >
            Accessibility:{" "}
            {status.accessibility ? "Granted" : "Not Granted"}
          </span>
        </div>
      </div>
      <p className="text-white/30 text-[11px] mt-2">
        Click &quot;Grant Access&quot; to trigger the permission prompt, or open
        System Settings &gt; Privacy &amp; Security manually.
      </p>
    </div>
  );
}

/** General tab — organized into Apple-style setting groups. */
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
  const [restartNeeded, setRestartNeeded] = useState(false);
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

  useEffect(() => {
    const unlisten = listen("hotkey-restart-required", () => {
      setRestartNeeded(true);
    });
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

  const handleHotkeyCapture = useCallback(
    (e: React.KeyboardEvent) => {
      e.preventDefault();
      e.stopPropagation();

      const parts: string[] = [];
      if (e.metaKey || e.ctrlKey) parts.push("CmdOrCtrl");
      if (e.shiftKey) parts.push("Shift");
      if (e.altKey) parts.push("Alt");

      // Use e.code (physical key) to avoid macOS Alt-composed characters
      // like ∫ (Alt+B), ß (Alt+S), etc. e.code gives "KeyB", "KeyS", etc.
      const code = e.code;
      if (
        code !== "MetaLeft" && code !== "MetaRight" &&
        code !== "ControlLeft" && code !== "ControlRight" &&
        code !== "ShiftLeft" && code !== "ShiftRight" &&
        code !== "AltLeft" && code !== "AltRight"
      ) {
        const codeMap: Record<string, string> = {
          Space: "Space",
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
        // e.code for letters is "KeyA"-"KeyZ", digits "Digit0"-"Digit9",
        // function keys "F1"-"F12", punctuation like "Period", "Comma", etc.
        let mappedKey = codeMap[code];
        if (!mappedKey) {
          if (code.startsWith("Key")) {
            mappedKey = code.slice(3); // "KeyB" -> "B"
          } else if (code.startsWith("Digit")) {
            mappedKey = code.slice(5); // "Digit1" -> "1"
          } else if (code.startsWith("Numpad")) {
            mappedKey = "num" + code.slice(6);
          } else {
            // Handle punctuation: "Period" -> ".", "Comma" -> ",", etc.
            const punctMap: Record<string, string> = {
              Period: ".", Comma: ",", Slash: "/", Backslash: "\\",
              BracketLeft: "[", BracketRight: "]", Semicolon: ";",
              Quote: "'", Backquote: "`", Minus: "-", Equal: "=",
            };
            mappedKey = punctMap[code] || code;
          }
        }
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
    <>
      {restartNeeded && (
        <div className="bg-[#FFCC00]/10 border border-[#FFCC00]/30 rounded-[10px] p-3 mb-5 flex items-center justify-between">
          <span className="text-[#FFCC00] text-[13px]">
            Restart to apply the new hotkey.
          </span>
          <button
            onClick={() => restartApp()}
            className="apple-button text-[12px]"
          >
            Restart Now
          </button>
        </div>
      )}

      {/* TRANSCRIPTION */}
      <SettingsGroup title="Transcription">
        <SettingsRow
          label="Backend"
          description={
            settings.transcription_backend === "Cloud"
              ? "Fast cloud transcription via Groq API"
              : "Private, on-device transcription"
          }
        >
          <select
            className="apple-select"
            value={settings.transcription_backend}
            onChange={(e) =>
              save({
                ...settings,
                transcription_backend: e.target.value as TranscriptionBackend,
              })
            }
          >
            <option value="Cloud">Cloud (Groq)</option>
            <option value="Local">Local (whisper.cpp)</option>
          </select>
        </SettingsRow>

        {settings.transcription_backend === "Local" && (
          <>
            <SettingsRow label="Whisper Model">
              <select
                className="apple-select"
                value={settings.model_size}
                onChange={(e) =>
                  save({
                    ...settings,
                    model_size: e.target.value as ModelSize,
                  })
                }
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
            </SettingsRow>

            {/* Model download status */}
            {models.length > 0 && (
              <div className="px-4 py-3 border-t border-white/10">
                {!isCurrentDownloaded && !downloading && (
                  <div className="flex items-center gap-2">
                    <button
                      onClick={handleDownload}
                      className="apple-button text-[12px]"
                    >
                      Download {settings.model_size}
                    </button>
                    <span className="text-white/30 text-[11px]">
                      From Hugging Face
                    </span>
                  </div>
                )}
                {downloading && (
                  <div className="space-y-1">
                    <div className="flex items-center gap-2">
                      <div className="flex-1 bg-white/10 rounded-full h-1.5">
                        <div
                          className="bg-[#007AFF] h-1.5 rounded-full transition-all"
                          style={{ width: `${progress}%` }}
                        />
                      </div>
                      <span className="text-[11px] text-white/55 w-10 text-right">
                        {progress}%
                      </span>
                    </div>
                    <p className="text-white/30 text-[11px]">
                      Downloading {downloading} model...
                    </p>
                  </div>
                )}
                {downloadError && (
                  <p className="text-[#FF3B30] text-[11px]">
                    Download failed: {downloadError}
                  </p>
                )}
                {isCurrentDownloaded && !downloading && (
                  <div className="flex items-center gap-2">
                    <span className="text-[#34C759] text-[11px]">
                      Model ready
                    </span>
                    <button
                      onClick={() => handleDeleteModel(settings.model_size)}
                      className="text-white/30 hover:text-[#FF3B30] text-[11px]"
                    >
                      Delete
                    </button>
                  </div>
                )}
              </div>
            )}
          </>
        )}

        <SettingsRow label="Language">
          <select
            className="apple-select"
            value={settings.language}
            onChange={(e) => save({ ...settings, language: e.target.value })}
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
        </SettingsRow>

        {(settings.transcription_backend === "Cloud" ||
          settings.rewrite_enabled) && (
          <SettingsRow
            label="Groq API Key"
            description={
              <>
                Get a free key at{" "}
                <a
                  href="https://console.groq.com"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-[#007AFF] hover:underline"
                >
                  console.groq.com
                </a>
              </>
            }
          >
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
              className="apple-input w-48"
            />
          </SettingsRow>
        )}
      </SettingsGroup>

      {/* Groq Usage */}
      {(settings.transcription_backend === "Cloud" ||
        settings.rewrite_enabled) && <GroqUsageCard />}

      {/* INPUT */}
      <SettingsGroup title="Input">
        <SettingsRow
          label="Hotkey"
          description={
            capturingHotkey
              ? "Press a modifier + key combination"
              : "Restart required after changing"
          }
        >
          <div className="flex items-center gap-1.5">
            <input
              ref={hotkeyRef}
              type="text"
              value={
                capturingHotkey ? "Press shortcut..." : settings.shortcut
              }
              readOnly={!capturingHotkey}
              onKeyDown={capturingHotkey ? handleHotkeyCapture : undefined}
              onBlur={() => setCapturingHotkey(false)}
              className={`apple-input w-40 text-center ${
                capturingHotkey ? "!border-[#007AFF]" : ""
              }`}
            />
            <button
              onClick={() => {
                setCapturingHotkey(true);
                setTimeout(() => hotkeyRef.current?.focus(), 50);
              }}
              className="apple-button-secondary text-[12px]"
            >
              {capturingHotkey ? "Cancel" : "Change"}
            </button>
          </div>
        </SettingsRow>

        <SettingsRow label="Mode">
          <select
            className="apple-select"
            value={settings.hotkey_mode}
            onChange={(e) =>
              save({ ...settings, hotkey_mode: e.target.value as any })
            }
          >
            <option value="PushToTalk">Push to Talk</option>
            <option value="Toggle">Toggle</option>
          </select>
        </SettingsRow>
      </SettingsGroup>

      {/* OUTPUT */}
      <SettingsGroup title="Output">
        <SettingsRow label="Injection Method">
          <select
            className="apple-select"
            value={settings.injection_method}
            onChange={(e) =>
              save({
                ...settings,
                injection_method: e.target.value as any,
              })
            }
          >
            <option value="Clipboard">Clipboard (Cmd+V)</option>
            <option value="Keyboard">Keyboard Simulation</option>
          </select>
        </SettingsRow>

        <SettingsRow
          label="Auto-copy"
          description="Copy to clipboard when using keyboard injection"
        >
          <input
            type="checkbox"
            className="apple-toggle"
            checked={settings.auto_copy}
            onChange={(e) =>
              save({ ...settings, auto_copy: e.target.checked })
            }
          />
        </SettingsRow>
      </SettingsGroup>

      {/* AI REWRITE */}
      <SettingsGroup title="AI Rewrite">
        <SettingsRow
          label="Enable"
          description={`Press ${settings.rewrite_hotkey} to polish text with AI`}
        >
          <input
            type="checkbox"
            className="apple-toggle"
            checked={settings.rewrite_enabled}
            onChange={(e) =>
              save({ ...settings, rewrite_enabled: e.target.checked })
            }
          />
        </SettingsRow>

        {settings.rewrite_enabled && (
          <SettingsRow label="Style">
            <select
              className="apple-select"
              value={settings.rewrite_style}
              onChange={(e) =>
                save({
                  ...settings,
                  rewrite_style: e.target.value as RewriteStyle,
                })
              }
            >
              <option value="Professional">Professional</option>
              <option value="Casual">Casual</option>
              <option value="Concise">Concise</option>
              <option value="Friendly">Friendly</option>
            </select>
          </SettingsRow>
        )}
      </SettingsGroup>

      {/* SYSTEM */}
      <SettingsGroup title="System">
        <SettingsRow label="Launch at login">
          <input
            type="checkbox"
            className="apple-toggle"
            checked={settings.auto_launch}
            onChange={(e) =>
              save({ ...settings, auto_launch: e.target.checked })
            }
          />
        </SettingsRow>
      </SettingsGroup>

      {/* FEEDBACK */}
      <SettingsGroup title="Feedback">
        <SettingsRow
          label="Show overlay"
          description="Display floating overlay during recording"
        >
          <input
            type="checkbox"
            className="apple-toggle"
            checked={settings.show_overlay}
            onChange={(e) =>
              save({ ...settings, show_overlay: e.target.checked })
            }
          />
        </SettingsRow>
        <SettingsRow
          label="Overlay position"
          description="Where to show the recording overlay"
        >
          <select
            className="apple-select"
            value={settings.overlay_position ?? "TopCenter"}
            disabled={!settings.show_overlay}
            onChange={(e) =>
              save({
                ...settings,
                overlay_position: e.target.value as OverlayPosition,
              })
            }
          >
            <option value="TopCenter">Top center</option>
            <option value="Inline">At cursor</option>
          </select>
        </SettingsRow>
        <SettingsRow
          label="Sound feedback"
          description="Play sound on start/stop"
        >
          <input
            type="checkbox"
            className="apple-toggle"
            checked={settings.sound_feedback}
            onChange={(e) =>
              save({ ...settings, sound_feedback: e.target.checked })
            }
          />
        </SettingsRow>
      </SettingsGroup>
    </>
  );
}

/** Audio tab — voice detection and feedback settings. */
function AudioTab({
  settings,
  save,
}: {
  settings: NonNullable<ReturnType<typeof useSettings>["settings"]>;
  save: ReturnType<typeof useSettings>["save"];
}) {
  const [devices, setDevices] = useState<AudioDevice[]>([]);

  useEffect(() => {
    listAudioDevices().then(setDevices).catch(console.error);
  }, []);

  const defaultDevice = devices.find((d) => d.is_default);

  return (
    <>
      <SettingsGroup title="Input Device">
        <SettingsRow
          label="Microphone"
          description={
            settings.audio_input_device
              ? "Using selected device"
              : `Using system default${defaultDevice ? ` (${defaultDevice.name})` : ""}`
          }
        >
          <select
            className="apple-select"
            value={settings.audio_input_device ?? ""}
            onChange={(e) =>
              save({
                ...settings,
                audio_input_device: e.target.value || null,
              })
            }
          >
            <option value="">System Default</option>
            {devices.map((d) => (
              <option key={d.name} value={d.name}>
                {d.name}{d.is_default ? " (default)" : ""}
              </option>
            ))}
          </select>
        </SettingsRow>
      </SettingsGroup>

      <SettingsGroup title="Voice Detection">
        <SettingsRow
          label={`Silence Threshold: ${settings.vad_threshold.toFixed(3)}`}
          description="Lower = more sensitive, higher = ignores quiet sounds"
        >
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
            className="w-32 accent-[#007AFF]"
          />
        </SettingsRow>
        <SettingsRow
          label={`Silence Timeout: ${settings.vad_silence_timeout.toFixed(0)}s`}
          description="How long to keep listening after you stop speaking"
        >
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
            className="w-32 accent-[#007AFF]"
          />
        </SettingsRow>
      </SettingsGroup>
    </>
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
    const unsubs = [
      listen("transcription-complete", refresh),
      listen("rewrite-complete", refresh),
    ];
    return () => {
      unsubs.forEach((p) => p.then((fn) => fn()));
    };
  }, [refresh]);

  if (!usage || !usage.updated_at) {
    return (
      <SettingsGroup>
        <div className="px-4 py-3">
          <p className="text-white/30 text-[11px]">
            Groq usage data will appear after your first API call.
          </p>
        </div>
      </SettingsGroup>
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
    pct > 30 ? "bg-[#34C759]" : pct > 10 ? "bg-[#FFCC00]" : "bg-[#FF3B30]";

  return (
    <SettingsGroup>
      <div className="px-4 py-3 space-y-2">
        <div className="flex items-center justify-between">
          <span className="text-[11px] font-medium text-white/55">
            Groq API Daily Usage
          </span>
          <button
            onClick={refresh}
            className="text-white/30 hover:text-white/55 text-[11px]"
          >
            Refresh
          </button>
        </div>

        {reqPct != null && (
          <div>
            <div className="flex justify-between text-[11px] text-white/55 mb-0.5">
              <span>Requests</span>
              <span>
                {usage.remaining_requests} / {usage.limit_requests}
                {usage.reset_requests && (
                  <span className="text-white/30 ml-1">
                    (resets in {usage.reset_requests})
                  </span>
                )}
              </span>
            </div>
            <div className="w-full bg-white/10 rounded-full h-1.5">
              <div
                className={`h-1.5 rounded-full transition-all ${barColor(reqPct)}`}
                style={{ width: `${reqPct}%` }}
              />
            </div>
          </div>
        )}

        {tokPct != null && (
          <div>
            <div className="flex justify-between text-[11px] text-white/55 mb-0.5">
              <span>Tokens</span>
              <span>
                {usage.remaining_tokens?.toLocaleString()} /{" "}
                {usage.limit_tokens?.toLocaleString()}
                {usage.reset_tokens && (
                  <span className="text-white/30 ml-1">
                    (resets in {usage.reset_tokens})
                  </span>
                )}
              </span>
            </div>
            <div className="w-full bg-white/10 rounded-full h-1.5">
              <div
                className={`h-1.5 rounded-full transition-all ${barColor(tokPct)}`}
                style={{ width: `${tokPct}%` }}
              />
            </div>
          </div>
        )}

        {usage.updated_at && (
          <p className="text-white/20 text-[10px]">
            Last updated: {new Date(usage.updated_at).toLocaleTimeString()}
          </p>
        )}
      </div>
    </SettingsGroup>
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
    <>
      <SettingsGroup>
        <div className="settings-row">
          <div className="flex items-center gap-3">
            <span
              className={`inline-block h-3 w-3 rounded-full ${
                status === "Licensed"
                  ? "bg-[#34C759]"
                  : status === "Invalid"
                  ? "bg-[#FF3B30]"
                  : "bg-white/30"
              }`}
            />
            <span className="text-[13px] font-medium text-white/85">
              {status === "Licensed"
                ? "Licensed"
                : status === "Invalid"
                ? "Invalid License"
                : "Free Version"}
            </span>
          </div>
        </div>
      </SettingsGroup>

      {status === "Licensed" ? (
        <SettingsGroup>
          <div className="px-4 py-3 space-y-3">
            <p className="text-white/55 text-[13px]">
              Your license is active on this machine. Cloud transcription and AI
              rewrite are unlocked.
            </p>
            <div className="flex items-center gap-3">
              <button
                onClick={handleDeactivate}
                disabled={loading}
                className="apple-button-secondary"
              >
                {loading ? "Deactivating..." : "Deactivate License"}
              </button>
              <span className="text-white/30 text-[11px]">
                Transfer to another machine
              </span>
            </div>
          </div>
        </SettingsGroup>
      ) : (
        <SettingsGroup>
          <div className="px-4 py-3 space-y-3">
            <p className="text-white/55 text-[13px]">
              Enter your license key to unlock cloud transcription and AI
              rewrite.
            </p>
            <div className="flex gap-2">
              <input
                type="text"
                value={key}
                onChange={(e) => setKey(e.target.value)}
                placeholder="Enter license key..."
                className="apple-input flex-1"
              />
              <button
                onClick={handleActivate}
                disabled={loading || !key.trim()}
                className="apple-button"
              >
                {loading ? "Activating..." : "Activate"}
              </button>
            </div>
            {error && (
              <p className="text-[#FF3B30] text-[11px]">{error}</p>
            )}
          </div>
        </SettingsGroup>
      )}
    </>
  );
}

/** About tab. */
function AboutTab() {
  const [updateInfo, setUpdateInfo] = useState<UpdateInfo | null>(null);
  const [checking, setChecking] = useState(false);

  useEffect(() => {
    getUpdateInfo().then(setUpdateInfo);

    const unlisten = listen<UpdateInfo>("update-available", (event) => {
      setUpdateInfo(event.payload);
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  const handleCheckForUpdates = async () => {
    setChecking(true);
    try {
      const result = await checkForUpdates();
      setUpdateInfo(result);
    } finally {
      setChecking(false);
    }
  };

  const openReleaseUrl = (url: string) => {
    invoke("open_url", { url });
  };

  return (
    <SettingsGroup>
      <div className="px-4 py-6 text-center">
        <p className="text-white/85 font-medium text-[15px]">
          ShhhType v0.1.0
        </p>
        <p className="text-white/55 text-[13px] mt-1">
          Voice-to-text developer tool for macOS.
        </p>
        <p className="text-white/55 text-[13px]">
          Built with Tauri + whisper.cpp + React.
        </p>

        {updateInfo && (
          <div className="mt-4 px-3 py-2 rounded-lg bg-[#34C759]/15 border border-[#34C759]/30">
            <p className="text-[#34C759] text-[13px] font-medium">
              New release available: {updateInfo.tag_name}
            </p>
            <button
              onClick={() => openReleaseUrl(updateInfo.html_url)}
              className="text-[#007AFF] text-[12px] mt-1 hover:underline cursor-pointer"
            >
              Download from GitHub
            </button>
          </div>
        )}

        <button
          onClick={handleCheckForUpdates}
          disabled={checking}
          className="apple-button-secondary mt-4"
        >
          {checking ? "Checking..." : "Check for Updates"}
        </button>
      </div>
    </SettingsGroup>
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
    return <p className="text-white/55 text-[13px]">Loading dictionary...</p>;
  }

  return (
    <>
      <p className="text-white/55 text-[11px] mb-4">
        Add corrections for terms Whisper often gets wrong (e.g., &quot;react
        native&quot; &rarr; &quot;React Native&quot;).
      </p>

      {entries.length > 0 && (
        <SettingsGroup>
          {entries.map((entry, i) => (
            <div key={i} className="settings-row">
              <div className="flex-1 flex gap-2 items-center">
                <input
                  value={entry.from}
                  onChange={(e) => updateRow(i, "from", e.target.value)}
                  placeholder="whisper output"
                  className="apple-input flex-1"
                />
                <span className="text-white/30">&rarr;</span>
                <input
                  value={entry.to}
                  onChange={(e) => updateRow(i, "to", e.target.value)}
                  placeholder="correction"
                  className="apple-input flex-1"
                />
              </div>
              <button
                onClick={() => removeRow(i)}
                className="text-white/30 hover:text-[#FF3B30] text-[11px] ml-2"
              >
                Remove
              </button>
            </div>
          ))}
        </SettingsGroup>
      )}

      <div className="flex gap-2 items-center">
        <button onClick={addRow} className="apple-button-secondary">
          + Add Entry
        </button>
        <button onClick={handleSave} className="apple-button">
          Save Dictionary
        </button>
        {saved && (
          <span className="text-[#34C759] text-[11px]">Saved!</span>
        )}
      </div>
    </>
  );
}
