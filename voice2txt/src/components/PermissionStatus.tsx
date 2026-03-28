import { useEffect, useState } from "react";
import { checkPermissions } from "../lib/commands";
import type { PermissionStatus as PermStatus } from "../lib/types";

/** Shows macOS permission status for microphone and accessibility. */
export default function PermissionStatus() {
  const [status, setStatus] = useState<PermStatus | null>(null);

  useEffect(() => {
    checkPermissions().then(setStatus).catch(console.error);
  }, []);

  if (!status) return null;

  const allGranted = status.microphone && status.accessibility;
  if (allGranted) return null;

  return (
    <div className="bg-yellow-900/50 border border-yellow-700 rounded p-4 space-y-3">
      <h3 className="text-yellow-300 font-medium text-sm">
        Permissions Required
      </h3>
      <div className="space-y-2 text-sm">
        <div className="flex items-center gap-2">
          <span
            className={`inline-block h-2 w-2 rounded-full ${
              status.microphone ? "bg-green-400" : "bg-red-400"
            }`}
          />
          <span className={status.microphone ? "text-green-300" : "text-red-300"}>
            Microphone: {status.microphone ? "Granted" : "Not Granted"}
          </span>
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
            Accessibility: {status.accessibility ? "Granted" : "Not Granted"}
          </span>
        </div>
      </div>
      <p className="text-gray-400 text-xs">
        Open System Settings to grant permissions.
      </p>
    </div>
  );
}
