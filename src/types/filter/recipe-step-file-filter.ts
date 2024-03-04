import type { FilterInterface, Order } from "../filter-interface.ts";

export type RecipeStepFileCondition = {
  recipeStepId?: number;
};

export type RecipeStepFileOrderBy = { order: Order };

export type RecipeStepFileFilter = FilterInterface<
  RecipeStepFileCondition,
  RecipeStepFileOrderBy
>;
