import { Component, onMount } from 'solid-js';

import { invoke} from "@tauri-apps/api";
const App: Component = () => {


  const createWindow = () => {
    invoke("spawn_window");
  }
  return (
    <div>
      <h1>Main window</h1>
      <button onClick={createWindow}> create window </button>
    </div>
  );
};

export default App;
