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

type Step = 1 | 2 | 3 | 4 | 5;

/** Welcome/onboarding wizard for first-time users. */
export default function Welcome() {
  const [step, setStep] = useState<Step>(1);
  const [permissions, setPermissions] = useState<PermissionStatus | null>(null);
  const [models, setModels] = useState<ModelStatus[]>([]);
  const [downloading, setDownloading] = useState(false);
  const [progress, setProgress] = useState(0);
  const [settings, setSettings] = useState<Settings | null>(null);
  const [licenseKey, setLicenseKey] = useState("");

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
    getSettings().then(setSettings).catch(console.error);
  }, []);

  // Listen for download progress
  useEffect(() => {
    const unlisten = listen<DownloadProgress>("model-download-progress", (event) => {
      setProgress(Math.round(event.payload.percent));
    });
    return () => { unlisten.then((fn) => fn()); };
  }, []);

  const handleRequestMic = async () => {
    await requestMicrophonePermission();
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

  const handleFinish = async () => {
    if (settings) {
      await updateSettings({ ...settings }).catch(console.error);
    }
    // Close the welcome window
    const { getCurrentWindow } = await import("@tauri-apps/api/window");
    getCurrentWindow().close();
  };

  return (
    <div className="settings-window min-h-screen p-8 flex flex-col">
      <h1 className="text-2xl font-bold mb-2">Welcome to ShhhType</h1>
      <p className="text-gray-400 mb-6">Let's get you set up in a few quick steps.</p>

      {/* Step indicators */}
      <div className="flex gap-2 mb-8">
        {[1, 2, 3, 4, 5].map((s) => (
          <div
            key={s}
            className={`h-1.5 flex-1 rounded-full ${
              s <= step ? "bg-blue-500" : "bg-gray-700"
            }`}
          />
        ))}
      </div>

      <div className="flex-1">
        {/* Step 1: Microphone */}
        {step === 1 && (
          <div className="space-y-4">
            <h2 className="text-lg font-medium">Microphone Access</h2>
            <p className="text-gray-400 text-sm">
              ShhhType needs microphone access to capture your voice.
            </p>
            <div className="flex items-center gap-3">
              <span
                className={`inline-block h-3 w-3 rounded-full ${
                  permissions?.microphone ? "bg-green-400" : "bg-red-400"
                }`}
              />
              <span className={permissions?.microphone ? "text-green-300" : "text-red-300"}>
                {permissions?.microphone ? "Microphone access granted" : "Microphone access needed"}
              </span>
            </div>
            {!permissions?.microphone && (
              <button
                onClick={handleRequestMic}
                className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded text-sm"
              >
                Grant Microphone Access
              </button>
            )}
          </div>
        )}

        {/* Step 2: Accessibility */}
        {step === 2 && (
          <div className="space-y-4">
            <h2 className="text-lg font-medium">Accessibility Permission</h2>
            <p className="text-gray-400 text-sm">
              ShhhType needs accessibility access to type text into your apps.
            </p>
            <div className="flex items-center gap-3">
              <span
                className={`inline-block h-3 w-3 rounded-full ${
                  permissions?.accessibility ? "bg-green-400" : "bg-red-400"
                }`}
              />
              <span className={permissions?.accessibility ? "text-green-300" : "text-red-300"}>
                {permissions?.accessibility ? "Accessibility access granted" : "Accessibility access needed"}
              </span>
            </div>
            {!permissions?.accessibility && (
              <div className="bg-gray-800 rounded p-4 text-sm text-gray-300 space-y-2">
                <p>To grant accessibility access:</p>
                <ol className="list-decimal list-inside space-y-1 text-gray-400">
                  <li>Open <strong>System Settings</strong></li>
                  <li>Go to <strong>Privacy & Security &gt; Accessibility</strong></li>
                  <li>Enable <strong>ShhhType</strong> in the list</li>
                </ol>
              </div>
            )}
          </div>
        )}

        {/* Step 3: Download Model */}
        {step === 3 && (
          <div className="space-y-4">
            <h2 className="text-lg font-medium">Download Speech Model</h2>
            <p className="text-gray-400 text-sm">
              The Base model (142MB) is recommended for a good balance of speed and accuracy.
            </p>
            {isBaseDownloaded ? (
              <div className="flex items-center gap-2">
                <span className="inline-block h-3 w-3 rounded-full bg-green-400" />
                <span className="text-green-300">Base model ready</span>
              </div>
            ) : downloading ? (
              <div className="space-y-2">
                <div className="flex items-center gap-2">
                  <div className="flex-1 bg-gray-700 rounded-full h-2">
                    <div
                      className="bg-blue-500 h-2 rounded-full transition-all"
                      style={{ width: `${progress}%` }}
                    />
                  </div>
                  <span className="text-xs text-gray-400 w-10 text-right">{progress}%</span>
                </div>
                <p className="text-gray-500 text-xs">Downloading Base model...</p>
              </div>
            ) : (
              <button
                onClick={handleDownloadModel}
                className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded text-sm"
              >
                Download Base Model (142MB)
              </button>
            )}
          </div>
        )}

        {/* Step 4: Quick Test */}
        {step === 4 && (
          <div className="space-y-4">
            <h2 className="text-lg font-medium">Quick Test</h2>
            <p className="text-gray-400 text-sm">
              You're all set! Try the global hotkey to record and transcribe.
            </p>
            <div className="bg-gray-800 rounded p-4 text-sm text-gray-300 space-y-2">
              <p>
                Press <kbd className="bg-gray-700 px-2 py-0.5 rounded text-white">{settings?.shortcut ?? "CmdOrCtrl+Alt+V"}</kbd> to start recording.
              </p>
              <p>Speak a few words, then release (Push-to-Talk) or press again (Toggle) to stop.</p>
              <p>The transcribed text will be typed into your currently focused app.</p>
            </div>
          </div>
        )}

        {/* Step 5: License (optional) */}
        {step === 5 && (
          <div className="space-y-4">
            <h2 className="text-lg font-medium">License Key (Optional)</h2>
            <p className="text-gray-400 text-sm">
              Enter a license key to unlock cloud transcription and AI rewrite features.
              You can skip this and use the free local-only version.
            </p>
            <input
              type="text"
              value={licenseKey}
              onChange={(e) => setLicenseKey(e.target.value)}
              placeholder="Enter license key..."
              className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-white placeholder-gray-600"
            />
            <p className="text-gray-500 text-xs">
              You can always add a license key later in Settings.
            </p>
          </div>
        )}
      </div>

      {/* Navigation buttons */}
      <div className="flex justify-between mt-8 pt-4 border-t border-gray-700">
        {step > 1 ? (
          <button
            onClick={() => setStep((s) => (s - 1) as Step)}
            className="text-gray-400 hover:text-white text-sm px-4 py-2"
          >
            Back
          </button>
        ) : (
          <div />
        )}
        {step < 5 ? (
          <button
            onClick={() => setStep((s) => (s + 1) as Step)}
            className="bg-blue-600 hover:bg-blue-700 text-white px-6 py-2 rounded text-sm"
          >
            Next
          </button>
        ) : (
          <button
            onClick={handleFinish}
            className="bg-green-600 hover:bg-green-700 text-white px-6 py-2 rounded text-sm"
          >
            Get Started
          </button>
        )}
      </div>
    </div>
  );
}
