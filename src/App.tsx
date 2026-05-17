import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { load } from "@tauri-apps/plugin-store";
import "./App.css";

function format(secs: number) {
  const mm = Math.floor(secs / 60);
  const ss = secs % 60;
  return `${String(mm).padStart(2, "0")}:${String(ss).padStart(2, "0")}`;
}

const STORE_KEY = "completed_today";

export default function App() {
  const [remaining, setRemaining] = useState(25 * 60);
  const [running, setRunning] = useState(false);
  const [completed, setCompleted] = useState(0);

  useEffect(() => {
    (async () => {
      const store = await load("tomato.json", { autoSave: true, defaults: {} });
      const val = (await store.get<number>(STORE_KEY)) ?? 0;
      setCompleted(val);
    })();

    const id = setInterval(async () => {
      const [r, isRunning] = await invoke<[number, boolean]>("tick");
      const wasRunning = running;
      setRemaining(r);
      setRunning(isRunning);
      if (wasRunning && !isRunning && r === 0) {
        const store = await load("tomato.json", { autoSave: true, defaults: {} });
        const next = ((await store.get<number>(STORE_KEY)) ?? 0) + 1;
        await store.set(STORE_KEY, next);
        setCompleted(next);
      }
    }, 500);
    return () => clearInterval(id);
  }, [running]);

  return (
    <main className="container">
      <div className="clock">{format(remaining)}</div>
      <div className="row">
        {!running ? (
          <button onClick={() => invoke("start")}>Start</button>
        ) : (
          <button onClick={() => invoke("pause")}>Pause</button>
        )}
        <button onClick={() => invoke("reset")}>Reset</button>
      </div>
      <div className="counter">Completed today: {completed}</div>
    </main>
  );
}
