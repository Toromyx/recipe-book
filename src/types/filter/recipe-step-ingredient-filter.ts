import type { FilterInterface, Order } from "../filter-interface.ts";

export type RecipeStepIngredientCondition = {
  recipeStepId?: number;
  ingredientId?: number;
};

export type RecipeStepIngredientOrderBy = { order: Order };

export type RecipeStepIngredientFilter = FilterInterface<
  RecipeStepIngredientCondition,
  RecipeStepIngredientOrderBy
>;
