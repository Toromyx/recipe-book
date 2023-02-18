import type {
  IngredientCreateInterface,
  IngredientInterface,
  IngredientUpdateInterface,
} from "../../types/entity/ingredient-interface.ts";
import type {
  RecipeIngredientCreateInterface,
  RecipeIngredientInterface,
  RecipeIngredientUpdateInterface,
} from "../../types/entity/recipe-ingredient-interface.ts";
import type {
  RecipeCreateInterface,
  RecipeInterface,
  RecipeUpdateInterface,
} from "../../types/entity/recipe-interface.ts";
import type {
  RecipeStepCreateInterface,
  RecipeStepInterface,
  RecipeStepUpdateInterface,
} from "../../types/entity/recipe-step-interface.ts";
import type { IngredientFilterInterface } from "../../types/filter/ingredient-filter-interface.ts";
import type { RecipeFilterInterface } from "../../types/filter/recipe-filter-interface.ts";
import type { RecipeIngredientFilterInterface } from "../../types/filter/recipe-ingredient-filter-interface.ts";
import type { RecipeStepFilterInterface } from "../../types/filter/recipe-step-filter-interface.ts";
import { client } from "./client.ts";
import type { CommandAnswer } from "./command-answer.ts";
import { Command } from "./command.ts";

type CommandEntityRead =
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
  [T in CommandEntityRead]: {
    [id: number]: Promise<CommandAnswer<T>>;
  };
} = {
  [Command.ENTITY_READ_INGREDIENT]: {},
  [Command.ENTITY_READ_RECIPE]: {},
  [Command.ENTITY_READ_RECIPE_INGREDIENT]: {},
  [Command.ENTITY_READ_RECIPE_STEP]: {},
};

function readCollected<T extends CommandEntityRead>(
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
  createIngredient(create: IngredientCreateInterface): Promise<number> {
    return client.invoke(Command.ENTITY_CREATE_INGREDIENT, { create });
  },
  readIngredient(id: number): Promise<IngredientInterface> {
    return readCollected(Command.ENTITY_READ_INGREDIENT, id);
  },
  updateIngredient(update: IngredientUpdateInterface): Promise<void> {
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

  createRecipe(create: RecipeCreateInterface): Promise<number> {
    return client.invoke(Command.ENTITY_CREATE_RECIPE, { create });
  },
  readRecipe(id: number): Promise<RecipeInterface> {
    return readCollected(Command.ENTITY_READ_RECIPE, id);
  },
  updateRecipe(update: RecipeUpdateInterface): Promise<void> {
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
    create: RecipeIngredientCreateInterface,
  ): Promise<number> {
    return client.invoke(Command.ENTITY_CREATE_RECIPE_INGREDIENT, { create });
  },
  readRecipeIngredient(id: number): Promise<RecipeIngredientInterface> {
    return readCollected(Command.ENTITY_READ_RECIPE_INGREDIENT, id);
  },
  updateRecipeIngredient(
    update: RecipeIngredientUpdateInterface,
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

  createRecipeStep(create: RecipeStepCreateInterface): Promise<number> {
    return client.invoke(Command.ENTITY_CREATE_RECIPE_STEP, { create });
  },
  readRecipeStep(id: number): Promise<RecipeStepInterface> {
    return readCollected(Command.ENTITY_READ_RECIPE_STEP, id);
  },
  updateRecipeStep(update: RecipeStepUpdateInterface): Promise<void> {
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
