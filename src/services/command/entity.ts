import type {
  IngredientCreateInterface,
  IngredientInterface,
  IngredientUpdateInterface,
} from "../../types/entity/ingredient-interface.ts";
import type {
  RecipeFileCreateInterface,
  RecipeFileInterface,
  RecipeFileUpdateInterface,
} from "../../types/entity/recipe-file-interface.ts";
import type {
  RecipeIngredientDraftCreateInterface,
  RecipeIngredientDraftInterface,
  RecipeIngredientDraftUpdateInterface,
} from "../../types/entity/recipe-ingredient-draft-interface.js";
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
import type { RecipeFileFilterInterface } from "../../types/filter/recipe-file-filter-interface.ts";
import type { RecipeFilterInterface } from "../../types/filter/recipe-filter-interface.ts";
import type { RecipeIngredientDraftFilterInterface } from "../../types/filter/recipe-ingredient-draft-filter-interface.js";
import type { RecipeIngredientFilterInterface } from "../../types/filter/recipe-ingredient-filter-interface.ts";
import type { RecipeStepFilterInterface } from "../../types/filter/recipe-step-filter-interface.ts";
import { stringifyFilter } from "../store/repository/entity-repository.ts";
import { invoke } from "./client.ts";
import type { CommandAnswer } from "./command-answer.ts";
import { Command } from "./command.ts";

type CommandEntityRead =
  | Command.ENTITY_READ_INGREDIENT
  | Command.ENTITY_READ_RECIPE
  | Command.ENTITY_READ_RECIPE_FILE
  | Command.ENTITY_READ_RECIPE_INGREDIENT
  | Command.ENTITY_READ_RECIPE_INGREDIENT_DRAFT
  | Command.ENTITY_READ_RECIPE_STEP;

type CommandEntityList =
  | Command.ENTITY_LIST_INGREDIENT
  | Command.ENTITY_LIST_RECIPE
  | Command.ENTITY_LIST_RECIPE_FILE
  | Command.ENTITY_LIST_RECIPE_INGREDIENT
  | Command.ENTITY_LIST_RECIPE_INGREDIENT_DRAFT
  | Command.ENTITY_LIST_RECIPE_STEP;

type CommandEntityCount =
  | Command.ENTITY_COUNT_INGREDIENT
  | Command.ENTITY_COUNT_RECIPE
  | Command.ENTITY_COUNT_RECIPE_FILE
  | Command.ENTITY_COUNT_RECIPE_INGREDIENT
  | Command.ENTITY_COUNT_RECIPE_INGREDIENT_DRAFT
  | Command.ENTITY_COUNT_RECIPE_STEP;

const entityReadPromiseCollector: {
  [T in CommandEntityRead]: {
    [id: number]: Promise<CommandAnswer<T>>;
  };
} = {
  [Command.ENTITY_READ_INGREDIENT]: {},
  [Command.ENTITY_READ_RECIPE]: {},
  [Command.ENTITY_READ_RECIPE_FILE]: {},
  [Command.ENTITY_READ_RECIPE_INGREDIENT]: {},
  [Command.ENTITY_READ_RECIPE_INGREDIENT_DRAFT]: {},
  [Command.ENTITY_READ_RECIPE_STEP]: {},
};

const entityListPromiseCollector: {
  [T in CommandEntityList]: {
    [filterKey: string]: Promise<CommandAnswer<T>>;
  };
} = {
  [Command.ENTITY_LIST_INGREDIENT]: {},
  [Command.ENTITY_LIST_RECIPE]: {},
  [Command.ENTITY_LIST_RECIPE_FILE]: {},
  [Command.ENTITY_LIST_RECIPE_INGREDIENT]: {},
  [Command.ENTITY_LIST_RECIPE_INGREDIENT_DRAFT]: {},
  [Command.ENTITY_LIST_RECIPE_STEP]: {},
};

const entityCountPromiseCollector: {
  [T in CommandEntityCount]: {
    [filterKey: string]: Promise<CommandAnswer<T>>;
  };
} = {
  [Command.ENTITY_COUNT_INGREDIENT]: {},
  [Command.ENTITY_COUNT_RECIPE]: {},
  [Command.ENTITY_COUNT_RECIPE_FILE]: {},
  [Command.ENTITY_COUNT_RECIPE_INGREDIENT]: {},
  [Command.ENTITY_COUNT_RECIPE_INGREDIENT_DRAFT]: {},
  [Command.ENTITY_COUNT_RECIPE_STEP]: {},
};

/**
 * Read an entity, returning the already existing promise if it already being read.
 */
function readCollected<T extends CommandEntityRead>(
  command: T,
  id: number,
): Promise<CommandAnswer<T>> {
  if (entityReadPromiseCollector[command][id] === undefined) {
    entityReadPromiseCollector[command][id] = invoke(command, { id }).then(
      (value: CommandAnswer<T>) => {
        delete entityReadPromiseCollector[command][id];
        return value;
      },
    );
  }
  return entityReadPromiseCollector[command][id];
}

/**
 * List entities, returning the already existing promise if they are already being listed.
 */
function listCollected<Filter extends object>(
  command: CommandEntityList,
  filter: Filter,
): Promise<CommandAnswer<typeof command>> {
  const filterKey = stringifyFilter(filter);
  if (entityListPromiseCollector[command][filterKey] === undefined) {
    entityListPromiseCollector[command][filterKey] = invoke(command, {
      filter,
    }).then((value: CommandAnswer<typeof command>) => {
      delete entityListPromiseCollector[command][filterKey];
      return value;
    });
  }
  return entityListPromiseCollector[command][filterKey];
}

/**
 * Count entities, returning the already existing promise if they are already being counted.
 */
function countCollected<Filter extends object>(
  command: CommandEntityCount,
  filter: Filter,
): Promise<CommandAnswer<typeof command>> {
  const filterKey = stringifyFilter(filter);
  if (entityCountPromiseCollector[command][filterKey] === undefined) {
    entityCountPromiseCollector[command][filterKey] = invoke(command, {
      filter,
    }).then((value: CommandAnswer<typeof command>) => {
      delete entityCountPromiseCollector[command][filterKey];
      return value;
    });
  }
  return entityCountPromiseCollector[command][filterKey];
}

export function createIngredient(
  create: IngredientCreateInterface,
): Promise<number> {
  return invoke(Command.ENTITY_CREATE_INGREDIENT, { create });
}

export function readIngredient(id: number): Promise<IngredientInterface> {
  return readCollected(Command.ENTITY_READ_INGREDIENT, id);
}

export function updateIngredient(
  update: IngredientUpdateInterface,
): Promise<void> {
  return invoke(Command.ENTITY_UPDATE_INGREDIENT, { update });
}

export function deleteIngredient(id: number): Promise<void> {
  return invoke(Command.ENTITY_DELETE_INGREDIENT, { id });
}

export function listIngredient(
  filter: IngredientFilterInterface,
): Promise<number[]> {
  return listCollected(Command.ENTITY_LIST_INGREDIENT, filter);
}

export function countIngredient(
  filter: IngredientFilterInterface,
): Promise<number> {
  return countCollected(Command.ENTITY_COUNT_INGREDIENT, filter);
}

export function createRecipe(create: RecipeCreateInterface): Promise<number> {
  return invoke(Command.ENTITY_CREATE_RECIPE, { create });
}

export function readRecipe(id: number): Promise<RecipeInterface> {
  return readCollected(Command.ENTITY_READ_RECIPE, id);
}

export function updateRecipe(update: RecipeUpdateInterface): Promise<void> {
  return invoke(Command.ENTITY_UPDATE_RECIPE, { update });
}

export function deleteRecipe(id: number): Promise<void> {
  return invoke(Command.ENTITY_DELETE_RECIPE, { id });
}

export function listRecipe(filter: RecipeFilterInterface): Promise<number[]> {
  return listCollected(Command.ENTITY_LIST_RECIPE, filter);
}

export function countRecipe(filter: RecipeFilterInterface): Promise<number> {
  return countCollected(Command.ENTITY_COUNT_RECIPE, filter);
}

export function createRecipeFile(
  create: RecipeFileCreateInterface,
): Promise<number> {
  return invoke(Command.ENTITY_CREATE_RECIPE_FILE, { create });
}

export function readRecipeFile(id: number): Promise<RecipeFileInterface> {
  return readCollected(Command.ENTITY_READ_RECIPE_FILE, id);
}

export function updateRecipeFile(
  update: RecipeFileUpdateInterface,
): Promise<void> {
  return invoke(Command.ENTITY_UPDATE_RECIPE_FILE, { update });
}

export function deleteRecipeFile(id: number): Promise<void> {
  return invoke(Command.ENTITY_DELETE_RECIPE_FILE, { id });
}

export function listRecipeFile(
  filter: RecipeFileFilterInterface,
): Promise<number[]> {
  return listCollected(Command.ENTITY_LIST_RECIPE_FILE, filter);
}

export function countRecipeFile(
  filter: RecipeFileFilterInterface,
): Promise<number> {
  return countCollected(Command.ENTITY_COUNT_RECIPE_FILE, filter);
}

export function createRecipeIngredient(
  create: RecipeIngredientCreateInterface,
): Promise<number> {
  return invoke(Command.ENTITY_CREATE_RECIPE_INGREDIENT, { create });
}

export function readRecipeIngredient(
  id: number,
): Promise<RecipeIngredientInterface> {
  return readCollected(Command.ENTITY_READ_RECIPE_INGREDIENT, id);
}

export function updateRecipeIngredient(
  update: RecipeIngredientUpdateInterface,
): Promise<void> {
  return invoke(Command.ENTITY_UPDATE_RECIPE_INGREDIENT, { update });
}

export function deleteRecipeIngredient(id: number): Promise<void> {
  return invoke(Command.ENTITY_DELETE_RECIPE_INGREDIENT, { id });
}

export function listRecipeIngredient(
  filter: RecipeIngredientFilterInterface,
): Promise<number[]> {
  return listCollected(Command.ENTITY_LIST_RECIPE_INGREDIENT, filter);
}

export function countRecipeIngredient(
  filter: RecipeIngredientFilterInterface,
): Promise<number> {
  return countCollected(Command.ENTITY_COUNT_RECIPE_INGREDIENT, filter);
}

export function createRecipeIngredientDraft(
  create: RecipeIngredientDraftCreateInterface,
): Promise<number> {
  return invoke(Command.ENTITY_CREATE_RECIPE_INGREDIENT_DRAFT, { create });
}

export function readRecipeIngredientDraft(
  id: number,
): Promise<RecipeIngredientDraftInterface> {
  return readCollected(Command.ENTITY_READ_RECIPE_INGREDIENT_DRAFT, id);
}

export function updateRecipeIngredientDraft(
  update: RecipeIngredientDraftUpdateInterface,
): Promise<void> {
  return invoke(Command.ENTITY_UPDATE_RECIPE_INGREDIENT_DRAFT, { update });
}

export function deleteRecipeIngredientDraft(id: number): Promise<void> {
  return invoke(Command.ENTITY_DELETE_RECIPE_INGREDIENT_DRAFT, { id });
}

export function listRecipeIngredientDraft(
  filter: RecipeIngredientDraftFilterInterface,
): Promise<number[]> {
  return listCollected(Command.ENTITY_LIST_RECIPE_INGREDIENT_DRAFT, filter);
}

export function countRecipeIngredientDraft(
  filter: RecipeIngredientDraftFilterInterface,
): Promise<number> {
  return countCollected(Command.ENTITY_COUNT_RECIPE_INGREDIENT_DRAFT, filter);
}

export function createRecipeStep(
  create: RecipeStepCreateInterface,
): Promise<number> {
  return invoke(Command.ENTITY_CREATE_RECIPE_STEP, { create });
}

export function readRecipeStep(id: number): Promise<RecipeStepInterface> {
  return readCollected(Command.ENTITY_READ_RECIPE_STEP, id);
}

export function updateRecipeStep(
  update: RecipeStepUpdateInterface,
): Promise<void> {
  return invoke(Command.ENTITY_UPDATE_RECIPE_STEP, { update });
}

export function deleteRecipeStep(id: number): Promise<void> {
  return invoke(Command.ENTITY_DELETE_RECIPE_STEP, { id });
}

export function listRecipeStep(
  filter: RecipeStepFilterInterface,
): Promise<number[]> {
  return listCollected(Command.ENTITY_LIST_RECIPE_STEP, filter);
}

export function countRecipeStep(
  filter: RecipeStepFilterInterface,
): Promise<number> {
  return countCollected(Command.ENTITY_COUNT_RECIPE_STEP, filter);
}
