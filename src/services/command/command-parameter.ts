import { Command } from "./command.js";
import { IngredientInterface } from "../../types/entity/ingredient-interface.js";
import { IngredientFilterInterface } from "../../types/filter/ingredient-filter-interface.js";
import { RecipeInterface } from "../../types/entity/recipe-interface.js";
import { RecipeFilterInterface } from "../../types/filter/recipe-filter-interface.js";
import { RecipeIngredientInterface } from "../../types/entity/recipe-ingredient-interface.js";
import { RecipeIngredientFilterInterface } from "../../types/filter/recipe-ingredient-filter-interface.js";
import { RecipeStepInterface } from "../../types/entity/recipe-step-interface.js";
import { RecipeStepFilterInterface } from "../../types/filter/recipe-step-filter-interface.js";

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
