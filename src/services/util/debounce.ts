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
