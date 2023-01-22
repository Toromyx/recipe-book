import type { FilterInterface } from "../filter-interface.js";

type RecipeStepCondition = {
  recipeId?: number;
};

type RecipeStepOrderBy = "order";

export type RecipeStepFilterInterface = FilterInterface<
  RecipeStepCondition,
  RecipeStepOrderBy
>;
