import type { FilterInterface, Order } from "../filter-interface.ts";

export type RecipeStepIngredientDraftCondition = {
  recipeStepId?: number;
};

export type RecipeStepIngredientDraftOrderBy = { order: Order };

export type RecipeStepIngredientDraftFilter = FilterInterface<
  RecipeStepIngredientDraftCondition,
  RecipeStepIngredientDraftOrderBy
>;
