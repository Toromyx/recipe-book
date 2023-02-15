import type { FilterInterface } from "../filter-interface.ts";

type RecipeIngredientCondition = {
  recipeStepId?: number;
  ingredientId?: number;
};

type RecipeIngredientOrderBy = "order";

export type RecipeIngredientFilterInterface = FilterInterface<
  RecipeIngredientCondition,
  RecipeIngredientOrderBy
>;
