import type { ActionReturn } from "svelte/action";

/**
 * @see https://github.com/sveltejs/svelte/issues/1133
 */
export function portal(node: HTMLElement, target: HTMLElement): ActionReturn {
  const childNodes = [...node.childNodes];
  target.append(...childNodes);

  return {
    destroy() {
      for (const childNode of childNodes) {
        childNode.remove();
      }
    },
  };
}
