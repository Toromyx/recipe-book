import type { FilterInterface, Order } from "../filter-interface.ts";

export type RecipeIngredientCondition = {
  recipeStepId?: number;
  ingredientId?: number;
};

export type RecipeIngredientOrderBy = { order: Order };

export type RecipeIngredientFilter = FilterInterface<
  RecipeIngredientCondition,
  RecipeIngredientOrderBy
>;
