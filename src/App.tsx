import { useEffect, useState } from "react";
import "./App.css";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

type Status = "unknown" | "up" | "down" | "degraded";

type Target = {
  id: string;
  name: string;
  check: {
    type: "http" | "tcp";
    url?: string;
    expected_status?: number;
    host?: string;
    port?: number;
  };
  interval_secs: number;
  timeout_ms: number;
};

type StatusUpdate = {
  id: string;
  status: Status;
  message: string | null;
};

function App() {
  const [targets, setTargets] = useState<Target[]>([]);
  const [statuses, setStatuses] = useState<Record<string, Status>>({});
  useEffect(() => {
    invoke<Target[]>("list_targets").then((targets) => {
      setTargets(targets);
    });

    const unlisten = listen<StatusUpdate>("status-changed", (event) => {
      console.log("Received event:", event.payload);
      setStatuses((prev) => ({
        ...prev,
        [event.payload.id]: event.payload.status,
      }));
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  return (
    <main className="panel">
      <h1>Healthbar Status</h1>
      <ul>
        {targets.map((target) => (
          <li key={target.id}>
            {target.name}: {statuses[target.id] || "unknown"}
          </li>
        ))}
      </ul>
    </main>
  );
}

export default App;
