import { invoke as tauriInvoke } from "@tauri-apps/api/tauri";
import { debug } from "../log.ts";
import type { CommandAnswer } from "./command-answer.ts";
import type { CommandParameter } from "./command-parameter.ts";
import type { Command } from "./command.ts";

/**
 * Invoke a tauri command.
 */
export async function invoke<T extends Command>(
  command: T,
  args: CommandParameter<T>,
): Promise<CommandAnswer<T>> {
  debug(`Invoked command "${command}".`, args);
  const answer = await tauriInvoke(command, args);
  debug(`Received command "${command}".`, answer);
  return answer as CommandAnswer<T>;
}
