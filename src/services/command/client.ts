import { debug } from "../log.js";
import { Command } from "./command.js";
import { CommandParameter } from "./command-parameter.js";
import { CommandAnswer } from "./command-answer.js";
import { invoke } from "@tauri-apps/api/tauri";

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
