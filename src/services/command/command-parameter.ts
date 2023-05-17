import type {
  IngredientCreateInterface,
  IngredientUpdateInterface,
} from "../../types/entity/ingredient-interface.ts";
import type {
  RecipeFileCreateInterface,
  RecipeFileUpdateInterface,
} from "../../types/entity/recipe-file-interface.ts";
import type {
  RecipeIngredientDraftCreateInterface,
  RecipeIngredientDraftUpdateInterface,
} from "../../types/entity/recipe-ingredient-draft-interface.js";
import type {
  RecipeIngredientCreateInterface,
  RecipeIngredientUpdateInterface,
} from "../../types/entity/recipe-ingredient-interface.ts";
import type {
  RecipeCreateInterface,
  RecipeUpdateInterface,
} from "../../types/entity/recipe-interface.ts";
import type {
  RecipeStepCreateInterface,
  RecipeStepUpdateInterface,
} from "../../types/entity/recipe-step-interface.ts";
import type { IngredientFilterInterface } from "../../types/filter/ingredient-filter-interface.ts";
import type { RecipeFileFilterInterface } from "../../types/filter/recipe-file-filter-interface.ts";
import type { RecipeFilterInterface } from "../../types/filter/recipe-filter-interface.ts";
import type { RecipeIngredientDraftFilterInterface } from "../../types/filter/recipe-ingredient-draft-filter-interface.js";
import type { RecipeIngredientFilterInterface } from "../../types/filter/recipe-ingredient-filter-interface.ts";
import type { RecipeStepFilterInterface } from "../../types/filter/recipe-step-filter-interface.ts";
import type { Command } from "./command.ts";

type CommandParameterMap = {
  [Command.ENTITY_CREATE_INGREDIENT]: { create: IngredientCreateInterface };
  [Command.ENTITY_READ_INGREDIENT]: { id: number };
  [Command.ENTITY_UPDATE_INGREDIENT]: { update: IngredientUpdateInterface };
  [Command.ENTITY_DELETE_INGREDIENT]: { id: number };
  [Command.ENTITY_LIST_INGREDIENT]: { filter: IngredientFilterInterface };
  [Command.ENTITY_COUNT_INGREDIENT]: { filter: IngredientFilterInterface };

  [Command.ENTITY_CREATE_RECIPE]: { create: RecipeCreateInterface };
  [Command.ENTITY_READ_RECIPE]: { id: number };
  [Command.ENTITY_UPDATE_RECIPE]: { update: RecipeUpdateInterface };
  [Command.ENTITY_DELETE_RECIPE]: { id: number };
  [Command.ENTITY_LIST_RECIPE]: { filter: RecipeFilterInterface };
  [Command.ENTITY_COUNT_RECIPE]: { filter: RecipeFilterInterface };

  [Command.ENTITY_CREATE_RECIPE_FILE]: {
    create: RecipeFileCreateInterface;
  };
  [Command.ENTITY_READ_RECIPE_FILE]: { id: number };
  [Command.ENTITY_UPDATE_RECIPE_FILE]: {
    update: RecipeFileUpdateInterface;
  };
  [Command.ENTITY_DELETE_RECIPE_FILE]: { id: number };
  [Command.ENTITY_LIST_RECIPE_FILE]: {
    filter: RecipeFileFilterInterface;
  };
  [Command.ENTITY_COUNT_RECIPE_FILE]: {
    filter: RecipeFileFilterInterface;
  };

  [Command.ENTITY_CREATE_RECIPE_INGREDIENT]: {
    create: RecipeIngredientCreateInterface;
  };
  [Command.ENTITY_READ_RECIPE_INGREDIENT]: { id: number };
  [Command.ENTITY_UPDATE_RECIPE_INGREDIENT]: {
    update: RecipeIngredientUpdateInterface;
  };
  [Command.ENTITY_DELETE_RECIPE_INGREDIENT]: { id: number };
  [Command.ENTITY_LIST_RECIPE_INGREDIENT]: {
    filter: RecipeIngredientFilterInterface;
  };
  [Command.ENTITY_COUNT_RECIPE_INGREDIENT]: {
    filter: RecipeIngredientFilterInterface;
  };

  [Command.ENTITY_CREATE_RECIPE_INGREDIENT_DRAFT]: {
    create: RecipeIngredientDraftCreateInterface;
  };
  [Command.ENTITY_READ_RECIPE_INGREDIENT_DRAFT]: { id: number };
  [Command.ENTITY_UPDATE_RECIPE_INGREDIENT_DRAFT]: {
    update: RecipeIngredientDraftUpdateInterface;
  };
  [Command.ENTITY_DELETE_RECIPE_INGREDIENT_DRAFT]: { id: number };
  [Command.ENTITY_LIST_RECIPE_INGREDIENT_DRAFT]: {
    filter: RecipeIngredientDraftFilterInterface;
  };
  [Command.ENTITY_COUNT_RECIPE_INGREDIENT_DRAFT]: {
    filter: RecipeIngredientDraftFilterInterface;
  };

  [Command.ENTITY_CREATE_RECIPE_STEP]: { create: RecipeStepCreateInterface };
  [Command.ENTITY_READ_RECIPE_STEP]: { id: number };
  [Command.ENTITY_UPDATE_RECIPE_STEP]: { update: RecipeStepUpdateInterface };
  [Command.ENTITY_DELETE_RECIPE_STEP]: { id: number };
  [Command.ENTITY_LIST_RECIPE_STEP]: { filter: RecipeStepFilterInterface };
  [Command.ENTITY_COUNT_RECIPE_STEP]: { filter: RecipeStepFilterInterface };

  [Command.EXTERNAL_RECIPE]: { url: string };

  [Command.OCR]: { recipeFileId: number };

  [Command.UNIT_LIST_GET]: undefined;
};
export type CommandParameter<T extends Command> = CommandParameterMap[T];
