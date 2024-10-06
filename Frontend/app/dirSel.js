const {invoke} = window.__TAURI__.tauri;

function getDir() {
  invoke("choose_directory");
}

