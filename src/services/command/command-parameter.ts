import type { Command } from "./command.ts";
import type { IngredientInterface } from "../../types/entity/ingredient-interface.ts";
import type { IngredientFilterInterface } from "../../types/filter/ingredient-filter-interface.ts";
import type { RecipeInterface } from "../../types/entity/recipe-interface.ts";
import type { RecipeFilterInterface } from "../../types/filter/recipe-filter-interface.ts";
import type { RecipeIngredientInterface } from "../../types/entity/recipe-ingredient-interface.ts";
import type { RecipeIngredientFilterInterface } from "../../types/filter/recipe-ingredient-filter-interface.ts";
import type { RecipeStepInterface } from "../../types/entity/recipe-step-interface.ts";
import type { RecipeStepFilterInterface } from "../../types/filter/recipe-step-filter-interface.ts";

type CommandParameterMap = {
  [Command.ENTITY_CREATE_INGREDIENT]: { create: Partial<IngredientInterface> };
  [Command.ENTITY_READ_INGREDIENT]: { id: number };
  [Command.ENTITY_UPDATE_INGREDIENT]: { update: Partial<IngredientInterface> };
  [Command.ENTITY_DELETE_INGREDIENT]: { id: number };
  [Command.ENTITY_LIST_INGREDIENT]: { filter: IngredientFilterInterface };
  [Command.ENTITY_COUNT_INGREDIENT]: { filter: IngredientFilterInterface };

  [Command.ENTITY_CREATE_RECIPE]: { create: Partial<RecipeInterface> };
  [Command.ENTITY_READ_RECIPE]: { id: number };
  [Command.ENTITY_UPDATE_RECIPE]: { update: Partial<RecipeInterface> };
  [Command.ENTITY_DELETE_RECIPE]: { id: number };
  [Command.ENTITY_LIST_RECIPE]: { filter: RecipeFilterInterface };
  [Command.ENTITY_COUNT_RECIPE]: { filter: RecipeFilterInterface };

  [Command.ENTITY_CREATE_RECIPE_INGREDIENT]: {
    create: Partial<RecipeIngredientInterface>;
  };
  [Command.ENTITY_READ_RECIPE_INGREDIENT]: { id: number };
  [Command.ENTITY_UPDATE_RECIPE_INGREDIENT]: {
    update: Partial<RecipeIngredientInterface>;
  };
  [Command.ENTITY_DELETE_RECIPE_INGREDIENT]: { id: number };
  [Command.ENTITY_LIST_RECIPE_INGREDIENT]: {
    filter: RecipeIngredientFilterInterface;
  };
  [Command.ENTITY_COUNT_RECIPE_INGREDIENT]: {
    filter: RecipeIngredientFilterInterface;
  };

  [Command.ENTITY_CREATE_RECIPE_STEP]: { create: Partial<RecipeStepInterface> };
  [Command.ENTITY_READ_RECIPE_STEP]: { id: number };
  [Command.ENTITY_UPDATE_RECIPE_STEP]: { update: Partial<RecipeStepInterface> };
  [Command.ENTITY_DELETE_RECIPE_STEP]: { id: number };
  [Command.ENTITY_LIST_RECIPE_STEP]: { filter: RecipeStepFilterInterface };
  [Command.ENTITY_COUNT_RECIPE_STEP]: { filter: RecipeStepFilterInterface };
};
export type CommandParameter<T extends Command> = CommandParameterMap[T];
