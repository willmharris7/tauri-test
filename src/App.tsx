import { useState } from "react";
import "./App.css";
import { getNextWindowPosition, setWindowPosition } from "./lib/moveWindow";
import { checkScreenpipeRunning } from "./lib/checkScreenpipeRunning";

function App() {
  const [posIndex, setPosIndex] = useState(0);
  const [screenpipeRunning, setScreenpipeRunning] = useState<boolean | null>(null);

  async function moveWindow() {
    const position = await getNextWindowPosition(posIndex);
    if (!position) return;
    await setWindowPosition(position);
    setPosIndex((i) => i === 3 ? 0 : i + 1); // loops the PhysicalPositions array 0-3
  }

  async function setScreenpipeStatus() {
    setScreenpipeRunning(await checkScreenpipeRunning());
  }

  return (
    <main>
      <button onClick={moveWindow}>Move window</button>
      <button onClick={setScreenpipeStatus}>Check screenpipe</button>
      {screenpipeRunning !== null && <p>Screenpipe running: {screenpipeRunning ? "yes" : "no"}</p>}
    </main>
  );
}

export default App;
