/**
 * Get the sum of multiple numbers.
 */
export function sum(...numbers: number[]): number {
  return numbers.reduce((sum, number) => sum + number, 0);
}

/**
 * Get the mean of multiple numbers.
 */
export function mean(...numbers: number[]): number {
  return sum(...numbers) / numbers.length;
}

/**
 * Get the standard deviation of multiple numbers.
 */
export function standardDeviation(...numbers: number[]): number {
  const m = mean(...numbers);
  return mean(...numbers.map((number) => (number - m) ** 2));
}
