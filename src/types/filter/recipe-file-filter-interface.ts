import type { FilterInterface } from "../filter-interface.ts";

type RecipeFileCondition = {
  recipeStepId?: number;
};

type RecipeFileOrderBy = "order";

export type RecipeFileFilterInterface = FilterInterface<
  RecipeFileCondition,
  RecipeFileOrderBy
>;
