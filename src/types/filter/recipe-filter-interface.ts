import { FilterInterface } from "../filter-interface.js";

type RecipeCondition = {
  name?: string;
};

type RecipeOrderBy = "name";

export type RecipeFilterInterface = FilterInterface<
  RecipeCondition,
  RecipeOrderBy
>;
