import "./services/polyfill.ts";
import App from "./App.svelte";

new App({
  target: document.getElementById("app") as HTMLElement,
});
