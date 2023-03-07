import type { Loadable } from "../../types/loadable.ts";

export function isLoading<T>(loadable: Loadable<T>): boolean {
  return loadable === undefined;
}
