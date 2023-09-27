import "./services/dom-content-loaded.ts";
import "./services/polyfill.ts";
import "./services/scraper.ts";
import App from "./App.svelte";

new App({
  target: document.getElementById("app") as HTMLElement,
});
