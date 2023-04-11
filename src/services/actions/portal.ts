import type { ActionReturn } from "svelte/action";

/**
 * Moves every child node of an element to a target element.
 *
 * @see https://github.com/sveltejs/svelte/issues/1133#issuecomment-682131447
 */
export function portal(
  element: HTMLElement,
  target: HTMLElement,
): ActionReturn {
  const childNodes = [...element.childNodes];
  target.append(...childNodes);

  return {
    destroy() {
      for (const childNode of childNodes) {
        childNode.remove();
      }
    },
  };
}
