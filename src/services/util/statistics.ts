export function sum(...numbers: number[]): number {
  return numbers.reduce((sum, number) => sum + number, 0);
}

export function mean(...numbers: number[]): number {
  return sum(...numbers) / numbers.length;
}

export function standardDeviation(...numbers: number[]): number {
  const m = mean(...numbers);
  return mean(...numbers.map((number) => (number - m) ** 2));
}
