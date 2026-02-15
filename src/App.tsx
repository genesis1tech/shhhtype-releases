import { getCurrentWindow } from "@tauri-apps/api/window";
import { useEffect, useState } from "react";
import Overlay from "./components/Overlay";
import Settings from "./components/Settings";
import "./styles/globals.css";

/** Route to the correct component based on window label. */
function App() {
  const [windowLabel, setWindowLabel] = useState<string | null>(null);

  useEffect(() => {
    const label = getCurrentWindow().label;
    setWindowLabel(label);
  }, []);

  if (windowLabel === null) return null;

  // The overlay window shows the floating recording pill
  if (windowLabel === "overlay") {
    return <Overlay />;
  }

  // Default: settings window
  return <Settings />;
}

export default App;
