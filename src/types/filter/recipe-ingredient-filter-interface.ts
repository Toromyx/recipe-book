import { FilterInterface } from "../filter-interface.js";

type RecipeIngredientCondition = {
  recipeStepId?: number;
  ingredientId?: number;
};

type RecipeIngredientOrderBy = "order";

export type RecipeIngredientFilterInterface = FilterInterface<
  RecipeIngredientCondition,
  RecipeIngredientOrderBy
>;
