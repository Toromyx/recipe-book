import type { ActionReturn } from "svelte/action";

/**
 * Moves every child of a node to a target node.
 *
 * @see https://github.com/sveltejs/svelte/issues/1133#issuecomment-682131447
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
