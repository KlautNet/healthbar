import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [uptime, setUptime] = useState<string | null>(null);

  const fetchUptime = async () => {
    try {
      const result = await invoke<string>("get_uptime");
      setUptime(result);
    } catch (error) {
      console.error("Error fetching uptime:", error);
    }
  };

  // Fetch uptime when the component mounts
  useState(() => {
    fetchUptime();
  });

  return (
    <main className="container">
      <p>{uptime}</p>
    </main>
  );
}

export default App;
