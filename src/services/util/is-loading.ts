import type { Loadable } from "../../types/loadable.ts";

export function isLoading<T>(loadable: Loadable<T>): loadable is undefined {
  return loadable === undefined;
}
