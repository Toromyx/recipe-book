import type { FilterInterface } from "../filter-interface.ts";

type RecipeCondition = {
  name?: string;
};

type RecipeOrderBy = "name";

export type RecipeFilterInterface = FilterInterface<
  RecipeCondition,
  RecipeOrderBy
>;
