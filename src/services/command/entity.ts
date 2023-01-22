import { client } from "./client.js";
import { Command } from "./command.js";
import type { CommandAnswer } from "./command-answer.js";
import type { IngredientInterface } from "../../types/entity/ingredient-interface.js";
import type { IngredientFilterInterface } from "../../types/filter/ingredient-filter-interface.js";
import type { RecipeInterface } from "../../types/entity/recipe-interface.js";
import type { RecipeFilterInterface } from "../../types/filter/recipe-filter-interface.js";
import type { RecipeIngredientInterface } from "../../types/entity/recipe-ingredient-interface.js";
import type { RecipeIngredientFilterInterface } from "../../types/filter/recipe-ingredient-filter-interface.js";
import type { RecipeStepInterface } from "../../types/entity/recipe-step-interface.js";
import type { RecipeStepFilterInterface } from "../../types/filter/recipe-step-filter-interface.js";

type ChannelEntityRead =
  | Command.ENTITY_READ_INGREDIENT
  | Command.ENTITY_READ_RECIPE
  | Command.ENTITY_READ_RECIPE_INGREDIENT
  | Command.ENTITY_READ_RECIPE_STEP;

/**
 * when there are duplicate requests to the same entity, the existing promise is returned
 *
 * this promise is deleted once resolved, this is not a cache!
 */
const entityReadPromiseCollector: {
  [channel in ChannelEntityRead]: {
    [id: number]: Promise<CommandAnswer<channel>>;
  };
} = {
  [Command.ENTITY_READ_INGREDIENT]: {},
  [Command.ENTITY_READ_RECIPE]: {},
  [Command.ENTITY_READ_RECIPE_INGREDIENT]: {},
  [Command.ENTITY_READ_RECIPE_STEP]: {},
};

function readCollected<T extends ChannelEntityRead>(
  command: T,
  id: number,
): Promise<CommandAnswer<T>> {
  if (entityReadPromiseCollector[command][id] === undefined) {
    entityReadPromiseCollector[command][id] = client
      .invoke(command, { id })
      .then((value: CommandAnswer<T>) => {
        delete entityReadPromiseCollector[command][id];
        return value;
      });
  }
  return entityReadPromiseCollector[command][id] as unknown as Promise<
    CommandAnswer<T>
  >;
}

export const apiClient = {
  createIngredient(create: Partial<IngredientInterface>): Promise<number> {
    return client.invoke(Command.ENTITY_CREATE_INGREDIENT, { create });
  },
  readIngredient(id: number): Promise<IngredientInterface> {
    return readCollected(Command.ENTITY_READ_INGREDIENT, id);
  },
  updateIngredient(update: Partial<IngredientInterface>): Promise<void> {
    return client.invoke(Command.ENTITY_UPDATE_INGREDIENT, { update });
  },
  deleteIngredient(id: number): Promise<void> {
    return client.invoke(Command.ENTITY_DELETE_INGREDIENT, { id });
  },
  listIngredient(filter: IngredientFilterInterface): Promise<number[]> {
    return client.invoke(Command.ENTITY_LIST_INGREDIENT, { filter });
  },
  countIngredient(filter: IngredientFilterInterface): Promise<number> {
    return client.invoke(Command.ENTITY_COUNT_INGREDIENT, { filter });
  },

  createRecipe(create: Partial<RecipeInterface>): Promise<number> {
    return client.invoke(Command.ENTITY_CREATE_RECIPE, { create });
  },
  readRecipe(id: number): Promise<RecipeInterface> {
    return readCollected(Command.ENTITY_READ_RECIPE, id);
  },
  updateRecipe(update: Partial<RecipeInterface>): Promise<void> {
    return client.invoke(Command.ENTITY_UPDATE_RECIPE, { update });
  },
  deleteRecipe(id: number): Promise<void> {
    return client.invoke(Command.ENTITY_DELETE_RECIPE, { id });
  },
  listRecipe(filter: RecipeFilterInterface): Promise<number[]> {
    return client.invoke(Command.ENTITY_LIST_RECIPE, { filter });
  },
  countRecipe(filter: RecipeFilterInterface): Promise<number> {
    return client.invoke(Command.ENTITY_COUNT_RECIPE, { filter });
  },

  createRecipeIngredient(
    create: Partial<RecipeIngredientInterface>,
  ): Promise<number> {
    return client.invoke(Command.ENTITY_CREATE_RECIPE_INGREDIENT, { create });
  },
  readRecipeIngredient(id: number): Promise<RecipeIngredientInterface> {
    return readCollected(Command.ENTITY_READ_RECIPE_INGREDIENT, id);
  },
  updateRecipeIngredient(
    update: Partial<RecipeIngredientInterface>,
  ): Promise<void> {
    return client.invoke(Command.ENTITY_UPDATE_RECIPE_INGREDIENT, { update });
  },
  deleteRecipeIngredient(id: number): Promise<void> {
    return client.invoke(Command.ENTITY_DELETE_RECIPE_INGREDIENT, { id });
  },
  listRecipeIngredient(
    filter: RecipeIngredientFilterInterface,
  ): Promise<number[]> {
    return client.invoke(Command.ENTITY_LIST_RECIPE_INGREDIENT, { filter });
  },
  countRecipeIngredient(
    filter: RecipeIngredientFilterInterface,
  ): Promise<number> {
    return client.invoke(Command.ENTITY_COUNT_RECIPE_INGREDIENT, { filter });
  },

  createRecipeStep(create: Partial<RecipeStepInterface>): Promise<number> {
    return client.invoke(Command.ENTITY_CREATE_RECIPE_STEP, { create });
  },
  readRecipeStep(id: number): Promise<RecipeStepInterface> {
    return readCollected(Command.ENTITY_READ_RECIPE_STEP, id);
  },
  updateRecipeStep(update: Partial<RecipeStepInterface>): Promise<void> {
    return client.invoke(Command.ENTITY_UPDATE_RECIPE_STEP, { update });
  },
  deleteRecipeStep(id: number): Promise<void> {
    return client.invoke(Command.ENTITY_DELETE_RECIPE_STEP, { id });
  },
  listRecipeStep(filter: RecipeStepFilterInterface): Promise<number[]> {
    return client.invoke(Command.ENTITY_LIST_RECIPE_STEP, { filter });
  },
  countRecipeStep(filter: RecipeStepFilterInterface): Promise<number> {
    return client.invoke(Command.ENTITY_COUNT_RECIPE_STEP, { filter });
  },
};
