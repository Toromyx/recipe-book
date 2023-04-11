import { listen, TauriEvent } from "@tauri-apps/api/event";
import type { ActionReturn } from "svelte/action";

let fileDropPayload: string[] | undefined = undefined;

window.addEventListener("mousemove", () => {
  fileDropPayload = undefined;
});

export function fileDrop(element: HTMLElement): ActionReturn {
  const onMouseEnter = () => {
    if (fileDropPayload) {
      element.dispatchEvent(
        new CustomEvent("fileDrop", { detail: fileDropPayload }),
      );
      fileDropPayload = undefined;
    }
  };
  element.addEventListener("mouseenter", onMouseEnter);
  const unListenFnPromise = listen(TauriEvent.WINDOW_FILE_DROP, (event) => {
    fileDropPayload = event.payload as string[];
  });

  return {
    destroy() {
      void unListenFnPromise.then((unListenFn) => unListenFn());
      element.removeEventListener("mouseenter", onMouseEnter);
    },
  };
}
