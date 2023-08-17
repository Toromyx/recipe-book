import { invoke } from "./command/client.ts";
import type { CommandError } from "./command/command-error.ts";
import { Command } from "./command/command.ts";

export class ExternalRecipeUrlNotSupportedError extends Error {
  url: string;

  constructor(url: string, message?: string) {
    super(message);
    this.url = url;
  }
}

export async function getExternalRecipe(url: string) {
  let recipeId;
  try {
    recipeId = await invoke(Command.EXTERNAL_RECIPE, {
      url,
    });
  } catch (reason) {
    const commandError: CommandError = reason as CommandError;
    if ("ExternalRecipeUrlNotSupported" in commandError) {
      throw new ExternalRecipeUrlNotSupportedError(
        commandError.ExternalRecipeUrlNotSupported,
      );
    }
    throw reason;
  }
  return recipeId;
}
