import type { FilterInterface, Order } from "../filter-interface.ts";

export type RecipeCondition = {
  name?: string;
};

export type RecipeOrderBy = { name: Order };

export type RecipeFilter = FilterInterface<RecipeCondition, RecipeOrderBy>;
