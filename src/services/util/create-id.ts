import { v4 } from "uuid";

export function createId(): string {
  return v4();
}
