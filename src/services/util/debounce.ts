/**
 * Create a trailing debounced version of the given asynchronous function by wait amount of milliseconds.
 *
 * The next execution always awaits the previous execution before adding the wait time on top.
 */
export function debounce(
  func: (...args: unknown[]) => Promise<void>,
  wait: number,
): typeof func {
  let timeout: number | undefined;
  let promise = Promise.resolve();
  return async (...args: Parameters<typeof func>) => {
    await promise;
    clearTimeout(timeout);
    timeout = setTimeout(() => {
      timeout = undefined;
      promise = func(...args);
    }, wait);
  };
}
