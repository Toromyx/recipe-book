import type { FilterInterface } from "../filter-interface.ts";

export type UnitNameCondition = object;

export type UnitNameOrderBy = object;

export type UnitNameFilter = FilterInterface<
  UnitNameCondition,
  UnitNameOrderBy
>;
