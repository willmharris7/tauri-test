import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

function App() {
  const [prompt, setPrompt] = useState("");
  const [output, setOutput] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");
  const [screenpipeReady, setScreenpipeReady] = useState(false);

  useEffect(() => {
    const unlisten = listen<boolean>("screenpipe-ready", (event) => {
      setScreenpipeReady(event.payload);
    });
    return () => { unlisten.then(f => f()); };
  }, []);

  async function sendPrompt() {
    if (!prompt.trim()) return;
    setLoading(true);
    setError("");
    setOutput("");
    try {
      const result = await invoke<string>("ask_claude", { prompt });
      setOutput(result);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);  
    }
  }

  if (!screenpipeReady) {
    return <main><p>Starting screenpipe...</p></main>;
  }

  return (
    <main>
      <div className="chat">
        <textarea
          className="input"
          value={prompt}
          onChange={(e) => setPrompt(e.target.value)}
          placeholder="Ask Claude..."
          rows={4}
          disabled={loading}
        />
        <button onClick={sendPrompt} disabled={loading || !prompt.trim()}>
          {loading ? "Thinking..." : "Send"}
        </button>
        {error && <pre className="output error">{error}</pre>}
        {output && <pre className="output">{output}</pre>}
      </div>
    </main>
  );
}

export default App;
