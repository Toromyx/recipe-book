import { FilterInterface } from "../filter-interface.js";

type IngredientCondition = {
  name?: string;
};

type IngredientOrderBy = "name";

export type IngredientFilterInterface = FilterInterface<
  IngredientCondition,
  IngredientOrderBy
>;
