import type { FilterInterface, Order } from "../filter-interface.ts";

export type RecipeStepCondition = {
  recipeId?: number;
};

export type RecipeStepOrderBy = { order: Order };

export type RecipeStepFilter = FilterInterface<
  RecipeStepCondition,
  RecipeStepOrderBy
>;
