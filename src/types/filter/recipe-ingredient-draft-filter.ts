import type { FilterInterface, Order } from "../filter-interface.ts";

export type RecipeIngredientDraftCondition = {
  recipeId?: number;
};

export type RecipeIngredientDraftOrderBy = { order: Order };

export type RecipeIngredientDraftFilter = FilterInterface<
  RecipeIngredientDraftCondition,
  RecipeIngredientDraftOrderBy
>;
