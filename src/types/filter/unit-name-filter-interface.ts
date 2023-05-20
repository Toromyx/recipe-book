import type { FilterInterface } from "../filter-interface.ts";

type UnitNameCondition = {
  nameExact?: string;
};

type UnitNameOrderBy = "name";

export type UnitNameFilterInterface = FilterInterface<
  UnitNameCondition,
  UnitNameOrderBy
>;
