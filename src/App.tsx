import { useState } from "react";
import { getCurrentWindow, currentMonitor } from "@tauri-apps/api/window";
import "./App.css";
import { PhysicalPosition } from "@tauri-apps/api/dpi";

function App() {
  const [posIndex, setPosIndex] = useState(0);

  async function moveWindow() {
    const win = getCurrentWindow();
    const monitor = await currentMonitor();
    if (!monitor) return;

    const { width, height } = monitor.size;
    const winSize = await win.outerSize();

    const positions = [
      new PhysicalPosition(0, 0),
      new PhysicalPosition(width - winSize.width, 0),
      new PhysicalPosition(width - winSize.width, height - winSize.height),
      new PhysicalPosition(0, height - winSize.height),
    ];

    await win.setPosition(positions[posIndex % 4]);
    setPosIndex((i) => i + 1);
  }

  return (
    <main>
      <button onClick={moveWindow}>Move window</button>
    </main>
  );
}

export default App;
