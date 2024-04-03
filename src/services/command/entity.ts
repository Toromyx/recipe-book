import type {
  FileCreateInterface,
  FileInterface,
  FileUpdateInterface,
} from "../../types/entity/file-interface.ts";
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
} from "../../types/entity/recipe-ingredient-draft-interface.ts";
import type {
  RecipeCreateInterface,
  RecipeInterface,
  RecipeUpdateInterface,
} from "../../types/entity/recipe-interface.ts";
import type {
  RecipeStepFileCreateInterface,
  RecipeStepFileInterface,
  RecipeStepFileUpdateInterface,
} from "../../types/entity/recipe-step-file-interface.ts";
import type {
  RecipeStepIngredientDraftCreateInterface,
  RecipeStepIngredientDraftInterface,
  RecipeStepIngredientDraftUpdateInterface,
} from "../../types/entity/recipe-step-ingredient-draft-interface.ts";
import type {
  RecipeStepIngredientCreateInterface,
  RecipeStepIngredientInterface,
  RecipeStepIngredientUpdateInterface,
} from "../../types/entity/recipe-step-ingredient-interface.ts";
import type {
  RecipeStepCreateInterface,
  RecipeStepInterface,
  RecipeStepUpdateInterface,
} from "../../types/entity/recipe-step-interface.ts";
import type {
  UnitNameCreateInterface,
  UnitNameInterface,
  UnitNameUpdateInterface,
} from "../../types/entity/unit-name-interface.ts";
import type {
  FileCondition,
  FileFilter,
} from "../../types/filter/file-filter.ts";
import type {
  IngredientCondition,
  IngredientFilter,
} from "../../types/filter/ingredient-filter.ts";
import type {
  RecipeFileCondition,
  RecipeFileFilter,
} from "../../types/filter/recipe-file-filter.ts";
import type {
  RecipeCondition,
  RecipeFilter,
} from "../../types/filter/recipe-filter.ts";
import type {
  RecipeIngredientDraftCondition,
  RecipeIngredientDraftFilter,
} from "../../types/filter/recipe-ingredient-draft-filter.ts";
import type {
  RecipeStepFileCondition,
  RecipeStepFileFilter,
} from "../../types/filter/recipe-step-file-filter.ts";
import type {
  RecipeStepCondition,
  RecipeStepFilter,
} from "../../types/filter/recipe-step-filter.ts";
import type {
  RecipeStepIngredientDraftCondition,
  RecipeStepIngredientDraftFilter,
} from "../../types/filter/recipe-step-ingredient-draft-filter.ts";
import type {
  RecipeStepIngredientCondition,
  RecipeStepIngredientFilter,
} from "../../types/filter/recipe-step-ingredient-filter.ts";
import type {
  UnitNameCondition,
  UnitNameFilter,
} from "../../types/filter/unit-name-filter.ts";
import { stringifyFilter } from "../store/repository/entity-repository.ts";
import { invoke } from "./client.ts";
import type { CommandAnswer } from "./command-answer.ts";
import type { CommandParameter } from "./command-parameter.ts";
import { Command } from "./command.ts";

type CommandEntityRead =
  | Command.ENTITY_READ_FILE
  | Command.ENTITY_READ_INGREDIENT
  | Command.ENTITY_READ_RECIPE
  | Command.ENTITY_READ_RECIPE_FILE
  | Command.ENTITY_READ_RECIPE_INGREDIENT_DRAFT
  | Command.ENTITY_READ_RECIPE_STEP_FILE
  | Command.ENTITY_READ_RECIPE_STEP_INGREDIENT
  | Command.ENTITY_READ_RECIPE_STEP_INGREDIENT_DRAFT
  | Command.ENTITY_READ_RECIPE_STEP
  | Command.ENTITY_READ_UNIT_NAME;

type CommandEntityList =
  | Command.ENTITY_LIST_FILE
  | Command.ENTITY_LIST_INGREDIENT
  | Command.ENTITY_LIST_RECIPE
  | Command.ENTITY_LIST_RECIPE_FILE
  | Command.ENTITY_LIST_RECIPE_INGREDIENT_DRAFT
  | Command.ENTITY_LIST_RECIPE_STEP_FILE
  | Command.ENTITY_LIST_RECIPE_STEP_INGREDIENT
  | Command.ENTITY_LIST_RECIPE_STEP_INGREDIENT_DRAFT
  | Command.ENTITY_LIST_RECIPE_STEP
  | Command.ENTITY_LIST_UNIT_NAME;

type CommandEntityCount =
  | Command.ENTITY_COUNT_FILE
  | Command.ENTITY_COUNT_INGREDIENT
  | Command.ENTITY_COUNT_RECIPE
  | Command.ENTITY_COUNT_RECIPE_FILE
  | Command.ENTITY_COUNT_RECIPE_INGREDIENT_DRAFT
  | Command.ENTITY_COUNT_RECIPE_STEP_FILE
  | Command.ENTITY_COUNT_RECIPE_STEP_INGREDIENT
  | Command.ENTITY_COUNT_RECIPE_STEP_INGREDIENT_DRAFT
  | Command.ENTITY_COUNT_RECIPE_STEP
  | Command.ENTITY_COUNT_UNIT_NAME;

const entityReadPromiseCollector: {
  [T in CommandEntityRead]: {
    [id in CommandParameter<T>["id"]]: Promise<CommandAnswer<T>>;
  };
} = {
  [Command.ENTITY_READ_FILE]: {},
  [Command.ENTITY_READ_INGREDIENT]: {},
  [Command.ENTITY_READ_RECIPE]: {},
  [Command.ENTITY_READ_RECIPE_FILE]: {},
  [Command.ENTITY_READ_RECIPE_INGREDIENT_DRAFT]: {},
  [Command.ENTITY_READ_RECIPE_STEP_FILE]: {},
  [Command.ENTITY_READ_RECIPE_STEP_INGREDIENT]: {},
  [Command.ENTITY_READ_RECIPE_STEP_INGREDIENT_DRAFT]: {},
  [Command.ENTITY_READ_RECIPE_STEP]: {},
  [Command.ENTITY_READ_UNIT_NAME]: {},
};

const entityListPromiseCollector: {
  [T in CommandEntityList]: {
    [filterKey: string]: Promise<CommandAnswer<T>>;
  };
} = {
  [Command.ENTITY_LIST_FILE]: {},
  [Command.ENTITY_LIST_INGREDIENT]: {},
  [Command.ENTITY_LIST_RECIPE]: {},
  [Command.ENTITY_LIST_RECIPE_FILE]: {},
  [Command.ENTITY_LIST_RECIPE_INGREDIENT_DRAFT]: {},
  [Command.ENTITY_LIST_RECIPE_STEP_FILE]: {},
  [Command.ENTITY_LIST_RECIPE_STEP_INGREDIENT]: {},
  [Command.ENTITY_LIST_RECIPE_STEP_INGREDIENT_DRAFT]: {},
  [Command.ENTITY_LIST_RECIPE_STEP]: {},
  [Command.ENTITY_LIST_UNIT_NAME]: {},
};

const entityCountPromiseCollector: {
  [T in CommandEntityCount]: {
    [conditionKey: string]: Promise<CommandAnswer<T>>;
  };
} = {
  [Command.ENTITY_COUNT_FILE]: {},
  [Command.ENTITY_COUNT_INGREDIENT]: {},
  [Command.ENTITY_COUNT_RECIPE]: {},
  [Command.ENTITY_COUNT_RECIPE_FILE]: {},
  [Command.ENTITY_COUNT_RECIPE_INGREDIENT_DRAFT]: {},
  [Command.ENTITY_COUNT_RECIPE_STEP_FILE]: {},
  [Command.ENTITY_COUNT_RECIPE_STEP_INGREDIENT]: {},
  [Command.ENTITY_COUNT_RECIPE_STEP_INGREDIENT_DRAFT]: {},
  [Command.ENTITY_COUNT_RECIPE_STEP]: {},
  [Command.ENTITY_COUNT_UNIT_NAME]: {},
};

/**
 * Read an entity, returning the already existing promise if it is already being read.
 */
function readCollected<Command extends CommandEntityRead>(
  command: Command,
  id: CommandParameter<Command>["id"],
): Promise<CommandAnswer<Command>> {
  if (entityReadPromiseCollector[command][id] === undefined) {
    // @ts-expect-error TypeScript gods help me
    entityReadPromiseCollector[command][id] = invoke(command, { id }).then(
      (value: CommandAnswer<Command>) => {
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
function listCollected<Command extends CommandEntityList>(
  command: Command,
  filter: CommandParameter<Command>["filter"],
): Promise<CommandAnswer<typeof command>> {
  const filterKey = stringifyFilter(filter);
  if (entityListPromiseCollector[command][filterKey] === undefined) {
    // @ts-expect-error TypeScript gods help me
    entityListPromiseCollector[command][filterKey] = invoke(command, {
      filter,
    } as CommandParameter<Command>).then(
      (value: CommandAnswer<typeof command>) => {
        delete entityListPromiseCollector[command][filterKey];
        return value;
      },
    );
  }
  return entityListPromiseCollector[command][filterKey];
}

/**
 * Count entities, returning the already existing promise if they are already being counted.
 */
function countCollected<Command extends CommandEntityCount>(
  command: Command,
  condition: CommandParameter<Command>["condition"],
): Promise<CommandAnswer<typeof command>> {
  const conditionKey = stringifyFilter(condition);
  if (entityCountPromiseCollector[command][conditionKey] === undefined) {
    // eslint-disable-next-line @typescript-eslint/no-unnecessary-type-assertion
    entityCountPromiseCollector[command as CommandEntityCount][conditionKey] =
      invoke(command, {
        condition,
      } as CommandParameter<Command>).then(
        (value: CommandAnswer<typeof command>) => {
          delete entityCountPromiseCollector[command][conditionKey];
          return value;
        },
      );
  }
  return entityCountPromiseCollector[command][conditionKey];
}

export function createFile(create: FileCreateInterface): Promise<number> {
  return invoke(Command.ENTITY_CREATE_FILE, { create });
}

export function readFile(id: number): Promise<FileInterface> {
  return readCollected(Command.ENTITY_READ_FILE, id);
}

export function updateFile(update: FileUpdateInterface): Promise<void> {
  return invoke(Command.ENTITY_UPDATE_FILE, { update });
}

export function deleteFile(id: number): Promise<void> {
  return invoke(Command.ENTITY_DELETE_FILE, { id });
}

export function listFile(filter: FileFilter): Promise<number[]> {
  return listCollected(Command.ENTITY_LIST_FILE, filter);
}

export function countFile(condition?: FileCondition): Promise<number> {
  return countCollected(Command.ENTITY_COUNT_FILE, condition);
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

export function listIngredient(filter: IngredientFilter): Promise<number[]> {
  return listCollected(Command.ENTITY_LIST_INGREDIENT, filter);
}

export function countIngredient(
  condition?: IngredientCondition,
): Promise<number> {
  return countCollected(Command.ENTITY_COUNT_INGREDIENT, condition);
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

export function listRecipe(filter: RecipeFilter): Promise<number[]> {
  return listCollected(Command.ENTITY_LIST_RECIPE, filter);
}

export function countRecipe(condition?: RecipeCondition): Promise<number> {
  return countCollected(Command.ENTITY_COUNT_RECIPE, condition);
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

export function listRecipeFile(filter: RecipeFileFilter): Promise<number[]> {
  return listCollected(Command.ENTITY_LIST_RECIPE_FILE, filter);
}

export function countRecipeFile(
  condition?: RecipeFileCondition,
): Promise<number> {
  return countCollected(Command.ENTITY_COUNT_RECIPE_FILE, condition);
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
  filter: RecipeIngredientDraftFilter,
): Promise<number[]> {
  return listCollected(Command.ENTITY_LIST_RECIPE_INGREDIENT_DRAFT, filter);
}

export function countRecipeIngredientDraft(
  condition?: RecipeIngredientDraftCondition,
): Promise<number> {
  return countCollected(
    Command.ENTITY_COUNT_RECIPE_INGREDIENT_DRAFT,
    condition,
  );
}

export function createRecipeStepFile(
  create: RecipeStepFileCreateInterface,
): Promise<number> {
  return invoke(Command.ENTITY_CREATE_RECIPE_STEP_FILE, { create });
}

export function readRecipeStepFile(
  id: number,
): Promise<RecipeStepFileInterface> {
  return readCollected(Command.ENTITY_READ_RECIPE_STEP_FILE, id);
}

export function updateRecipeStepFile(
  update: RecipeStepFileUpdateInterface,
): Promise<void> {
  return invoke(Command.ENTITY_UPDATE_RECIPE_STEP_FILE, { update });
}

export function deleteRecipeStepFile(id: number): Promise<void> {
  return invoke(Command.ENTITY_DELETE_RECIPE_STEP_FILE, { id });
}

export function listRecipeStepFile(
  filter: RecipeStepFileFilter,
): Promise<number[]> {
  return listCollected(Command.ENTITY_LIST_RECIPE_STEP_FILE, filter);
}

export function countRecipeStepFile(
  condition?: RecipeStepFileCondition,
): Promise<number> {
  return countCollected(Command.ENTITY_COUNT_RECIPE_STEP_FILE, condition);
}

export function createRecipeStepIngredient(
  create: RecipeStepIngredientCreateInterface,
): Promise<number> {
  return invoke(Command.ENTITY_CREATE_RECIPE_STEP_INGREDIENT, { create });
}

export function readRecipeStepIngredient(
  id: number,
): Promise<RecipeStepIngredientInterface> {
  return readCollected(Command.ENTITY_READ_RECIPE_STEP_INGREDIENT, id);
}

export function updateRecipeStepIngredient(
  update: RecipeStepIngredientUpdateInterface,
): Promise<void> {
  return invoke(Command.ENTITY_UPDATE_RECIPE_STEP_INGREDIENT, { update });
}

export function deleteRecipeStepIngredient(id: number): Promise<void> {
  return invoke(Command.ENTITY_DELETE_RECIPE_STEP_INGREDIENT, { id });
}

export function listRecipeStepIngredient(
  filter: RecipeStepIngredientFilter,
): Promise<number[]> {
  return listCollected(Command.ENTITY_LIST_RECIPE_STEP_INGREDIENT, filter);
}

export function countRecipeStepIngredient(
  condition?: RecipeStepIngredientCondition,
): Promise<number> {
  return countCollected(Command.ENTITY_COUNT_RECIPE_STEP_INGREDIENT, condition);
}

export function createRecipeStepIngredientDraft(
  create: RecipeStepIngredientDraftCreateInterface,
): Promise<number> {
  return invoke(Command.ENTITY_CREATE_RECIPE_STEP_INGREDIENT_DRAFT, { create });
}

export function readRecipeStepIngredientDraft(
  id: number,
): Promise<RecipeStepIngredientDraftInterface> {
  return readCollected(Command.ENTITY_READ_RECIPE_STEP_INGREDIENT_DRAFT, id);
}

export function updateRecipeStepIngredientDraft(
  update: RecipeStepIngredientDraftUpdateInterface,
): Promise<void> {
  return invoke(Command.ENTITY_UPDATE_RECIPE_STEP_INGREDIENT_DRAFT, { update });
}

export function deleteRecipeStepIngredientDraft(id: number): Promise<void> {
  return invoke(Command.ENTITY_DELETE_RECIPE_STEP_INGREDIENT_DRAFT, { id });
}

export function listRecipeStepIngredientDraft(
  filter: RecipeStepIngredientDraftFilter,
): Promise<number[]> {
  return listCollected(
    Command.ENTITY_LIST_RECIPE_STEP_INGREDIENT_DRAFT,
    filter,
  );
}

export function countRecipeStepIngredientDraft(
  condition?: RecipeStepIngredientDraftCondition,
): Promise<number> {
  return countCollected(
    Command.ENTITY_COUNT_RECIPE_STEP_INGREDIENT_DRAFT,
    condition,
  );
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

export function listRecipeStep(filter: RecipeStepFilter): Promise<number[]> {
  return listCollected(Command.ENTITY_LIST_RECIPE_STEP, filter);
}

export function countRecipeStep(
  condition?: RecipeStepCondition,
): Promise<number> {
  return countCollected(Command.ENTITY_COUNT_RECIPE_STEP, condition);
}

export function createUnitName(
  create: UnitNameCreateInterface,
): Promise<string> {
  return invoke(Command.ENTITY_CREATE_UNIT_NAME, { create });
}

export function readUnitName(id: string): Promise<UnitNameInterface> {
  return readCollected(Command.ENTITY_READ_UNIT_NAME, id);
}

export function updateUnitName(update: UnitNameUpdateInterface): Promise<void> {
  return invoke(Command.ENTITY_UPDATE_UNIT_NAME, { update });
}

export function deleteUnitName(id: string): Promise<void> {
  return invoke(Command.ENTITY_DELETE_UNIT_NAME, { id });
}

export function listUnitName(filter: UnitNameFilter): Promise<string[]> {
  return listCollected(Command.ENTITY_LIST_UNIT_NAME, filter);
}

export function countUnitName(condition?: UnitNameCondition): Promise<number> {
  return countCollected(Command.ENTITY_COUNT_UNIT_NAME, condition);
}
