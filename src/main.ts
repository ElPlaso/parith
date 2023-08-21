import { invoke } from "@tauri-apps/api/tauri";

let inputElement: HTMLInputElement | null;
let outputElement: HTMLElement | null;

async function run() {
  if (inputElement && outputElement) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    outputElement.textContent = await invoke("run", {
      input: inputElement.value,
    });
  }
}

window.addEventListener("DOMContentLoaded", () => {
  inputElement = document.querySelector("#input");
  outputElement = document.querySelector("#output");
  document.querySelector("#form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    run();
  });
});
