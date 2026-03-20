import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import {
  checkPermissions,
  requestMicrophonePermission,
  getModelStatus,
  downloadModel,
  getSettings,
  updateSettings,
} from "../lib/commands";
import type { PermissionStatus, ModelStatus, DownloadProgress, Settings } from "../lib/types";

type Step = 1 | 2 | 3 | 4;

/** Welcome/onboarding wizard for first-time users. */
export default function Welcome() {
  const [step, setStep] = useState<Step>(1);
  const [permissions, setPermissions] = useState<PermissionStatus | null>(null);
  const [models, setModels] = useState<ModelStatus[]>([]);
  const [downloading, setDownloading] = useState(false);
  const [progress, setProgress] = useState(0);
  const [settings, setSettings] = useState<Settings | null>(null);
  const [groqKey, setGroqKey] = useState("");
  const [micRequested, setMicRequested] = useState(false);

  // Poll permissions
  useEffect(() => {
    const refresh = () => checkPermissions().then(setPermissions).catch(console.error);
    refresh();
    const interval = setInterval(refresh, 2000);
    return () => clearInterval(interval);
  }, []);

  // Load models and settings
  useEffect(() => {
    getModelStatus().then(setModels).catch(console.error);
    getSettings().then((s) => {
      setSettings(s);
      if (s.groq_api_key) setGroqKey(s.groq_api_key);
    }).catch(console.error);
  }, []);

  // Listen for download progress
  useEffect(() => {
    const unlisten = listen<DownloadProgress>("model-download-progress", (event) => {
      setProgress(Math.round(event.payload.percent));
    });
    return () => { unlisten.then((fn) => fn()); };
  }, []);

  const handleRequestMic = async () => {
    setMicRequested(true);
    await requestMicrophonePermission();
    setTimeout(() => {
      checkPermissions().then(setPermissions).catch(console.error);
    }, 1500);
  };

  const handleSaveGroqKey = async () => {
    if (settings && groqKey.trim()) {
      const updated = {
        ...settings,
        groq_api_key: groqKey.trim(),
        transcription_backend: "Cloud" as const,
      };
      await updateSettings(updated).catch(console.error);
      setSettings(updated);
    }
  };

  const handleDownloadModel = async () => {
    setDownloading(true);
    setProgress(0);
    try {
      await downloadModel("Base");
      const updated = await getModelStatus();
      setModels(updated);
    } catch (e) {
      console.error("Download failed:", e);
    } finally {
      setDownloading(false);
    }
  };

  const baseModel = models.find((m) => m.model === "Base");
  const isBaseDownloaded = baseModel?.downloaded ?? false;
  const hasGroqKey = !!(settings?.groq_api_key || groqKey.trim());

  const handleFinish = async () => {
    if (settings && groqKey.trim() && settings.groq_api_key !== groqKey.trim()) {
      await updateSettings({
        ...settings,
        groq_api_key: groqKey.trim(),
        transcription_backend: "Cloud" as const,
      }).catch(console.error);
    }
    const { getCurrentWindow } = await import("@tauri-apps/api/window");
    getCurrentWindow().close();
  };

  return (
    <div className="settings-window min-h-screen flex flex-col">
      {/* Titlebar drag area */}
      <div className="window-drag h-[52px] shrink-0" />

      <div className="flex-1 px-8 pb-8 flex flex-col">
        <h1 className="text-[22px] font-bold text-white/85 mb-1">Welcome to ShhhType</h1>
        <p className="text-white/55 text-[13px] mb-6">Let's get you set up in a few quick steps.</p>

        {/* Step indicators (dots) */}
        <div className="flex gap-2 mb-8 justify-center">
          {[1, 2, 3, 4].map((s) => (
            <div
              key={s}
              className={`h-2 w-2 rounded-full transition-colors ${
                s <= step ? "bg-[#007AFF]" : "bg-white/20"
              }`}
            />
          ))}
        </div>

        <div className="flex-1">
          {/* Step 1: Microphone */}
          {step === 1 && (
            <div className="space-y-4">
              <h2 className="text-[17px] font-medium text-white/85">Microphone Access</h2>
              <p className="text-white/55 text-[13px]">
                ShhhType needs microphone access to capture your voice.
              </p>
              <div className="flex items-center gap-3">
                <span
                  className={`inline-block h-3 w-3 rounded-full ${
                    permissions?.microphone ? "bg-[#34C759]" : "bg-[#FF3B30]"
                  }`}
                />
                <span className={`text-[13px] ${permissions?.microphone ? "text-[#34C759]" : "text-[#FF3B30]"}`}>
                  {permissions?.microphone ? "Microphone access granted" : "Microphone access needed"}
                </span>
              </div>
              {!permissions?.microphone && !micRequested && (
                <button onClick={handleRequestMic} className="apple-button window-no-drag">
                  Grant Microphone Access
                </button>
              )}
              {!permissions?.microphone && micRequested && (
                <div className="bg-white/5 rounded-[10px] p-4 text-[13px] text-white/70 space-y-3">
                  <p>
                    If the permission prompt didn't appear, macOS needs you to grant it manually:
                  </p>
                  <ol className="list-decimal list-inside space-y-1 text-white/55">
                    <li>Open <strong>System Settings</strong></li>
                    <li>Go to <strong>Privacy &amp; Security &gt; Microphone</strong></li>
                    <li>Enable <strong>ShhhType</strong> in the list</li>
                  </ol>
                  <button
                    onClick={handleRequestMic}
                    className="apple-button-secondary window-no-drag"
                  >
                    Try Again
                  </button>
                </div>
              )}
            </div>
          )}

          {/* Step 2: Accessibility */}
          {step === 2 && (
            <div className="space-y-4">
              <h2 className="text-[17px] font-medium text-white/85">Accessibility Permission</h2>
              <p className="text-white/55 text-[13px]">
                ShhhType needs accessibility access to type text into your apps.
              </p>
              <div className="flex items-center gap-3">
                <span
                  className={`inline-block h-3 w-3 rounded-full ${
                    permissions?.accessibility ? "bg-[#34C759]" : "bg-[#FF3B30]"
                  }`}
                />
                <span className={`text-[13px] ${permissions?.accessibility ? "text-[#34C759]" : "text-[#FF3B30]"}`}>
                  {permissions?.accessibility ? "Accessibility access granted" : "Accessibility access needed"}
                </span>
              </div>
              {!permissions?.accessibility && (
                <div className="bg-white/5 rounded-[10px] p-4 text-[13px] text-white/70 space-y-2">
                  <p>To grant accessibility access:</p>
                  <ol className="list-decimal list-inside space-y-1 text-white/55">
                    <li>Open <strong>System Settings</strong></li>
                    <li>Go to <strong>Privacy &amp; Security &gt; Accessibility</strong></li>
                    <li>Enable <strong>ShhhType</strong> in the list</li>
                  </ol>
                </div>
              )}
            </div>
          )}

          {/* Step 3: Transcription Setup */}
          {step === 3 && (
            <div className="space-y-5">
              <h2 className="text-[17px] font-medium text-white/85">Transcription Setup</h2>

              {/* Primary: Groq API Key */}
              <div className="space-y-3">
                <p className="text-white/55 text-[13px]">
                  For the fastest transcription, enter a free Groq API key.
                  Groq uses whisper-large-v3-turbo in the cloud — fast and accurate.
                </p>
                <div>
                  <label className="block text-[13px] text-white/55 mb-1">
                    Groq API Key
                  </label>
                  <input
                    type="password"
                    value={groqKey}
                    onChange={(e) => setGroqKey(e.target.value)}
                    placeholder="gsk_..."
                    className="apple-input w-full window-no-drag"
                  />
                </div>
                {groqKey.trim() && (
                  <button onClick={handleSaveGroqKey} className="apple-button window-no-drag">
                    Save API Key
                  </button>
                )}
                {settings?.groq_api_key && (
                  <div className="flex items-center gap-2">
                    <span className="inline-block h-3 w-3 rounded-full bg-[#34C759]" />
                    <span className="text-[#34C759] text-[13px]">API key saved</span>
                  </div>
                )}
                <p className="text-white/30 text-[11px]">
                  Get a free API key at{" "}
                  <a
                    href="https://console.groq.com/keys"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-[#007AFF] hover:underline"
                  >
                    console.groq.com/keys
                  </a>
                </p>
              </div>

              {/* Divider */}
              <div className="flex items-center gap-3">
                <hr className="flex-1 border-white/10" />
                <span className="text-white/30 text-[11px]">OR</span>
                <hr className="flex-1 border-white/10" />
              </div>

              {/* Secondary: Local Model */}
              <div className="space-y-3">
                <p className="text-white/55 text-[13px]">
                  Prefer fully offline? Download a local Whisper model instead. No data leaves your machine.
                </p>
                {isBaseDownloaded ? (
                  <div className="flex items-center gap-2">
                    <span className="inline-block h-3 w-3 rounded-full bg-[#34C759]" />
                    <span className="text-[#34C759] text-[13px]">Base model ready</span>
                  </div>
                ) : downloading ? (
                  <div className="space-y-2">
                    <div className="flex items-center gap-2">
                      <div className="flex-1 bg-white/10 rounded-full h-1.5">
                        <div
                          className="bg-[#007AFF] h-1.5 rounded-full transition-all"
                          style={{ width: `${progress}%` }}
                        />
                      </div>
                      <span className="text-[11px] text-white/55 w-10 text-right">{progress}%</span>
                    </div>
                    <p className="text-white/30 text-[11px]">Downloading Base model...</p>
                  </div>
                ) : (
                  <button onClick={handleDownloadModel} className="apple-button-secondary window-no-drag">
                    Download Base Model (142MB)
                  </button>
                )}
              </div>
            </div>
          )}

          {/* Step 4: Quick Test */}
          {step === 4 && (
            <div className="space-y-4">
              <h2 className="text-[17px] font-medium text-white/85">You're All Set!</h2>
              <p className="text-white/55 text-[13px]">
                Try the global hotkey to record and transcribe.
              </p>
              <div className="bg-white/5 rounded-[10px] p-4 text-[13px] text-white/70 space-y-2">
                <p>
                  Press{" "}
                  <kbd className="bg-white/10 px-2 py-0.5 rounded text-white/85 text-[12px]">
                    {settings?.shortcut ?? "CmdOrCtrl+Alt+V"}
                  </kbd>{" "}
                  to start recording.
                </p>
                <p>Speak a few words, then release (Push-to-Talk) or press again (Toggle) to stop.</p>
                <p>The transcribed text will be typed into your currently focused app.</p>
              </div>
              {hasGroqKey && (
                <p className="text-white/30 text-[11px]">
                  Using Groq cloud transcription. Change to local mode anytime in Settings.
                </p>
              )}
            </div>
          )}
        </div>

        {/* Navigation buttons */}
        <div className="flex justify-between mt-8 pt-4 border-t border-white/10">
          {step > 1 ? (
            <button
              onClick={() => setStep((s) => (s - 1) as Step)}
              className="apple-button-secondary window-no-drag"
            >
              Back
            </button>
          ) : (
            <div />
          )}
          {step < 4 ? (
            <button
              onClick={() => setStep((s) => (s + 1) as Step)}
              className="apple-button window-no-drag"
            >
              Next
            </button>
          ) : (
            <button
              onClick={handleFinish}
              className="apple-button window-no-drag"
              style={{ background: "#34C759" }}
            >
              Get Started
            </button>
          )}
        </div>
      </div>
    </div>
  );
}
