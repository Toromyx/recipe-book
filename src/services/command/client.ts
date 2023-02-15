import { invoke } from "@tauri-apps/api/tauri";
import { debug } from "../log.ts";
import type { CommandAnswer } from "./command-answer.ts";
import type { CommandParameter } from "./command-parameter.ts";
import type { Command } from "./command.ts";

type CommandClient = {
  invoke<T extends Command>(
    command: T,
    args: CommandParameter<T>,
  ): Promise<CommandAnswer<T>>;
};

export const client: CommandClient = {
  async invoke<T extends Command>(
    command: T,
    args: CommandParameter<T>,
  ): Promise<CommandAnswer<T>> {
    debug(`Invoked command "${command}".`, args);
    const answer = await invoke(command, args);
    debug(`Received command "${command}".`, answer);
    return answer as CommandAnswer<T>;
  },
};
