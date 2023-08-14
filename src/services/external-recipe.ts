import { invoke } from "./command/client.ts";
import type { CommandError } from "./command/command-error.ts";
import { Command } from "./command/command.ts";

export class ExternalRecipeUrlNotSupportedError extends Error {}

export async function getExternalRecipe(url: string) {
  let recipeId;
  try {
    recipeId = await invoke(Command.EXTERNAL_RECIPE, {
      url,
    });
  } catch (reason) {
    if ("ExternalRecipeUrlNotSupported" in (reason as CommandError)) {
      throw new ExternalRecipeUrlNotSupportedError();
    }
    throw reason;
  }
  return recipeId;
}
