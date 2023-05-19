/**
 * Wraps a type to be in loaded or unloaded state.
 *
 * Currently unloaded just means undefined.
 */
export type Loadable<T> = T | undefined;

/**
 * @return true when the parameter is in the loading state
 */
export function isLoading<T>(loadable: Loadable<T>): loadable is undefined {
  return loadable === undefined;
}

/**
 * @return true when the parameter is in the loaded state
 */
export function isLoaded<T>(loadable: Loadable<T>): loadable is T {
  return loadable !== undefined;
}

/**
 * @return either the loadable value when loaded or the given default value if not
 */
export function whenLoadingDefault<T, D>(
  loadable: Loadable<T>,
  defaultValue: D,
): T | D {
  if (isLoading(loadable)) {
    return defaultValue;
  }

  return loadable;
}
