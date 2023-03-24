import type { Loadable } from "../../types/loadable.ts";

export function isLoading<T>(loadable: Loadable<T>): loadable is undefined {
  return loadable === undefined;
}

export function whenLoadingDefault<T, D>(
  loadable: Loadable<T>,
  defaultValue: D,
): T | D {
  if (isLoading(loadable)) {
    return defaultValue;
  }

  return loadable;
}
