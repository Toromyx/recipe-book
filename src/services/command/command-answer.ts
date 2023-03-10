import type { IngredientInterface } from "../../types/entity/ingredient-interface.ts";
import type { RecipeFileInterface } from "../../types/entity/recipe-file-interface.ts";
import type { RecipeIngredientInterface } from "../../types/entity/recipe-ingredient-interface.ts";
import type { RecipeInterface } from "../../types/entity/recipe-interface.ts";
import type { RecipeStepInterface } from "../../types/entity/recipe-step-interface.ts";
import type { Command } from "./command.ts";

type CommandAnswerMap = {
  [Command.ENTITY_CREATE_INGREDIENT]: number;
  [Command.ENTITY_READ_INGREDIENT]: IngredientInterface;
  [Command.ENTITY_UPDATE_INGREDIENT]: void;
  [Command.ENTITY_DELETE_INGREDIENT]: void;
  [Command.ENTITY_LIST_INGREDIENT]: number[];
  [Command.ENTITY_COUNT_INGREDIENT]: number;

  [Command.ENTITY_CREATE_RECIPE]: number;
  [Command.ENTITY_READ_RECIPE]: RecipeInterface;
  [Command.ENTITY_UPDATE_RECIPE]: void;
  [Command.ENTITY_DELETE_RECIPE]: void;
  [Command.ENTITY_LIST_RECIPE]: number[];
  [Command.ENTITY_COUNT_RECIPE]: number;

  [Command.ENTITY_CREATE_RECIPE_FILE]: number;
  [Command.ENTITY_READ_RECIPE_FILE]: RecipeFileInterface;
  [Command.ENTITY_UPDATE_RECIPE_FILE]: void;
  [Command.ENTITY_DELETE_RECIPE_FILE]: void;
  [Command.ENTITY_LIST_RECIPE_FILE]: number[];
  [Command.ENTITY_COUNT_RECIPE_FILE]: number;

  [Command.ENTITY_CREATE_RECIPE_INGREDIENT]: number;
  [Command.ENTITY_READ_RECIPE_INGREDIENT]: RecipeIngredientInterface;
  [Command.ENTITY_UPDATE_RECIPE_INGREDIENT]: void;
  [Command.ENTITY_DELETE_RECIPE_INGREDIENT]: void;
  [Command.ENTITY_LIST_RECIPE_INGREDIENT]: number[];
  [Command.ENTITY_COUNT_RECIPE_INGREDIENT]: number;

  [Command.ENTITY_CREATE_RECIPE_STEP]: number;
  [Command.ENTITY_READ_RECIPE_STEP]: RecipeStepInterface;
  [Command.ENTITY_UPDATE_RECIPE_STEP]: void;
  [Command.ENTITY_DELETE_RECIPE_STEP]: void;
  [Command.ENTITY_LIST_RECIPE_STEP]: number[];
  [Command.ENTITY_COUNT_RECIPE_STEP]: number;
};
export type CommandAnswer<T extends Command> = CommandAnswerMap[T];
