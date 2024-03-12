import type { FileInterface } from "../../types/entity/file-interface.ts";
import type { IngredientInterface } from "../../types/entity/ingredient-interface.ts";
import type { RecipeIngredientDraftInterface } from "../../types/entity/recipe-ingredient-draft-interface.ts";
import type { RecipeInterface } from "../../types/entity/recipe-interface.ts";
import type { RecipeStepFileInterface } from "../../types/entity/recipe-step-file-interface.ts";
import type { RecipeStepIngredientDraftInterface } from "../../types/entity/recipe-step-ingredient-draft-interface.ts";
import type { RecipeStepIngredientInterface } from "../../types/entity/recipe-step-ingredient-interface.ts";
import type { RecipeStepInterface } from "../../types/entity/recipe-step-interface.ts";
import type { UnitNameInterface } from "../../types/entity/unit-name-interface.ts";
import type { UnitConversion } from "../../types/unit-conversion.ts";
import type { Command } from "./command.ts";

type CommandAnswerMap = {
  [Command.ENTITY_CREATE_FILE]: number;
  [Command.ENTITY_READ_FILE]: FileInterface;
  [Command.ENTITY_UPDATE_FILE]: void;
  [Command.ENTITY_DELETE_FILE]: void;
  [Command.ENTITY_LIST_FILE]: number[];
  [Command.ENTITY_COUNT_FILE]: number;

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

  [Command.ENTITY_CREATE_RECIPE_INGREDIENT_DRAFT]: number;
  [Command.ENTITY_READ_RECIPE_INGREDIENT_DRAFT]: RecipeIngredientDraftInterface;
  [Command.ENTITY_UPDATE_RECIPE_INGREDIENT_DRAFT]: void;
  [Command.ENTITY_DELETE_RECIPE_INGREDIENT_DRAFT]: void;
  [Command.ENTITY_LIST_RECIPE_INGREDIENT_DRAFT]: number[];
  [Command.ENTITY_COUNT_RECIPE_INGREDIENT_DRAFT]: number;

  [Command.ENTITY_CREATE_RECIPE_STEP_FILE]: number;
  [Command.ENTITY_READ_RECIPE_STEP_FILE]: RecipeStepFileInterface;
  [Command.ENTITY_UPDATE_RECIPE_STEP_FILE]: void;
  [Command.ENTITY_DELETE_RECIPE_STEP_FILE]: void;
  [Command.ENTITY_LIST_RECIPE_STEP_FILE]: number[];
  [Command.ENTITY_COUNT_RECIPE_STEP_FILE]: number;

  [Command.ENTITY_CREATE_RECIPE_STEP_INGREDIENT]: number;
  [Command.ENTITY_READ_RECIPE_STEP_INGREDIENT]: RecipeStepIngredientInterface;
  [Command.ENTITY_UPDATE_RECIPE_STEP_INGREDIENT]: void;
  [Command.ENTITY_DELETE_RECIPE_STEP_INGREDIENT]: void;
  [Command.ENTITY_LIST_RECIPE_STEP_INGREDIENT]: number[];
  [Command.ENTITY_COUNT_RECIPE_STEP_INGREDIENT]: number;

  [Command.ENTITY_CREATE_RECIPE_STEP_INGREDIENT_DRAFT]: number;
  [Command.ENTITY_READ_RECIPE_STEP_INGREDIENT_DRAFT]: RecipeStepIngredientDraftInterface;
  [Command.ENTITY_UPDATE_RECIPE_STEP_INGREDIENT_DRAFT]: void;
  [Command.ENTITY_DELETE_RECIPE_STEP_INGREDIENT_DRAFT]: void;
  [Command.ENTITY_LIST_RECIPE_STEP_INGREDIENT_DRAFT]: number[];
  [Command.ENTITY_COUNT_RECIPE_STEP_INGREDIENT_DRAFT]: number;

  [Command.ENTITY_CREATE_RECIPE_STEP]: number;
  [Command.ENTITY_READ_RECIPE_STEP]: RecipeStepInterface;
  [Command.ENTITY_UPDATE_RECIPE_STEP]: void;
  [Command.ENTITY_DELETE_RECIPE_STEP]: void;
  [Command.ENTITY_LIST_RECIPE_STEP]: number[];
  [Command.ENTITY_COUNT_RECIPE_STEP]: number;

  [Command.ENTITY_CREATE_UNIT_NAME]: number;
  [Command.ENTITY_READ_UNIT_NAME]: UnitNameInterface;
  [Command.ENTITY_UPDATE_UNIT_NAME]: void;
  [Command.ENTITY_DELETE_UNIT_NAME]: void;
  [Command.ENTITY_LIST_UNIT_NAME]: number[];
  [Command.ENTITY_COUNT_UNIT_NAME]: number;

  [Command.EXTERNAL_RECIPE]: number;

  [Command.OCR]: string;

  [Command.UNIT_CONVERT]: UnitConversion;

  [Command.UNIT_LIST_GET]: string[];
};
export type CommandAnswer<T extends Command> = CommandAnswerMap[T];
