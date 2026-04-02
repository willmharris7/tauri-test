import { useState } from "react";
import { getCurrentWindow, currentMonitor } from "@tauri-apps/api/window";
import "./App.css";
import { PhysicalPosition } from "@tauri-apps/api/dpi";

function App() {
  const [posIndex, setPosIndex] = useState(0);

  async function moveWindow() {
    const window = getCurrentWindow();
    const windowSize = await window.outerSize(); // gets OS info of window size
    const windowWidth = windowSize.width
    const windowHeight = windowSize.height
    const monitor = await currentMonitor(); // gets OS level hardware info about the monitor
    if (!monitor) return;
    const monitorSize = monitor.size
    const monitorWidth = monitorSize.width;
    const monitorHeight = monitorSize.height;
    

    const positions = [
      new PhysicalPosition(0, 0),
      new PhysicalPosition(monitorWidth - windowWidth, 0),
      new PhysicalPosition(monitorWidth - windowWidth, monitorHeight - windowHeight ),
      new PhysicalPosition(0, monitorHeight - windowHeight ),
    ];

    await window.setPosition(positions[posIndex]);
    setPosIndex((i) => i === 3 ? 0 : i + 1); // loops the PhysicalPositions array 0-3
  }

  return (
    <main>
      <button onClick={moveWindow}>Move window</button>
    </main>
  );
}

export default App;
