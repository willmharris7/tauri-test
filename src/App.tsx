import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow, currentMonitor } from "@tauri-apps/api/window";
import { PhysicalPosition } from "@tauri-apps/api/dpi";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
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

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <main className="container">
      <h1>Welcome to Tauri + React</h1>

      <div className="row">
        <a href="https://vite.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>
      <button onClick={moveWindow}>Move window</button>
    </main>
  );
}

export default App;
