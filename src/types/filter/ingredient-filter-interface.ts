import type { FilterInterface } from "../filter-interface.ts";

type IngredientCondition = {
  name?: string;
};

type IngredientOrderBy = "name";

export type IngredientFilterInterface = FilterInterface<
  IngredientCondition,
  IngredientOrderBy
>;
