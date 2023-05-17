import type { FilterInterface } from "../filter-interface.ts";

type RecipeIngredientDraftCondition = {
  recipeStepId?: number;
};

type RecipeIngredientDraftOrderBy = "order";

export type RecipeIngredientDraftFilterInterface = FilterInterface<
  RecipeIngredientDraftCondition,
  RecipeIngredientDraftOrderBy
>;
