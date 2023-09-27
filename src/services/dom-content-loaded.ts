import { emit } from "@tauri-apps/api/event";

addEventListener("DOMContentLoaded", () => {
  void emit("DOMContentLoaded");
});
