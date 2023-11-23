import { createSignal } from "solid-js";

/* @ts-ignore */
import logo from "./assets/logo.svg";

import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [currentMac, setCurrentMac] = createSignal("");

  async function poll_current_mac() {
    setCurrentMac(await invoke("get_mac"));
    console.log("polling");
  }

  setCurrentMac("loading...");
  poll_current_mac();

  return (
    <div class="container">
      <h1>Welcome to codeswitch!</h1>

      <p>{currentMac()}</p>
    </div>
  );
}

export default App;
