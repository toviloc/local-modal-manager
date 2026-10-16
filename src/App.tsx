import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [space, setSpace] = useState<string>("");
  const [error, setError] = useState<string>("");
  async function loadSpace() {
    try {
      const bytes = await invoke<number>("get_disk_space");
      setSpace((bytes/1024**3).toFixed(1) + " GB free");
    }catch (e) {
      setError(String(e));
    }
  }

return (
  <main className="container">
    <button onClick={loadSpace}>Check Disk Space</button>
    {space && <p>{space}</p>}
    {error && <p style={{ color: "red" }}>{error}</p>}
  </main>
);
}

export default App;
