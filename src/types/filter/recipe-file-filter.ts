import type { FilterInterface, Order } from "../filter-interface.ts";

export type RecipeFileCondition = {
  recipeId?: number;
};

export type RecipeFileOrderBy = { order: Order };

export type RecipeFileFilter = FilterInterface<
  RecipeFileCondition,
  RecipeFileOrderBy
>;
