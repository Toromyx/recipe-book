import type { FilterInterface } from "../filter-interface.ts";

type RecipeStepCondition = {
  recipeId?: number;
};

type RecipeStepOrderBy = "order";

export type RecipeStepFilterInterface = FilterInterface<
  RecipeStepCondition,
  RecipeStepOrderBy
>;
