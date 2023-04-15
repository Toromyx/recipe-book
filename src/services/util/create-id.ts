import { v4 } from "uuid";

/**
 * Create a unique id for usage in HTML.
 */
export function createId(): string {
  return v4();
}
