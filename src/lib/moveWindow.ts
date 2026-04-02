import { getCurrentWindow, currentMonitor } from "@tauri-apps/api/window";
import { PhysicalPosition } from "@tauri-apps/api/dpi";

export async function getNextWindowPosition(posIndex: number): Promise<PhysicalPosition | null> {
  const window = getCurrentWindow();
  const { width: windowWidth, height: windowHeight } = await window.outerSize();
  const monitor = await currentMonitor();
  if (!monitor) return null;
  const { width: monitorWidth, height: monitorHeight} = monitor.size

  const positions = [
    new PhysicalPosition(0, 0),
    new PhysicalPosition(monitorWidth - windowWidth, 0),
    new PhysicalPosition(monitorWidth - windowWidth, monitorHeight - windowHeight),
    new PhysicalPosition(0, monitorHeight - windowHeight),
  ];

  return positions[posIndex];
}

export async function setWindowPosition(position: PhysicalPosition) {
  const window = getCurrentWindow();
  await window.setPosition(position);
}
