import type { FilterInterface, Order } from "../filter-interface.ts";

export type IngredientCondition = {
  name?: string;
  nameExact?: string;
  recipeStepId?: number;
};

export type IngredientOrderBy = { name: Order };

export type IngredientFilter = FilterInterface<
  IngredientCondition,
  IngredientOrderBy
>;
