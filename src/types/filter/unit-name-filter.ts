import type { FilterInterface, Order } from "../filter-interface.ts";

export type UnitNameCondition = {
  nameExact?: string;
};

export type UnitNameOrderBy = { name: Order };

export type UnitNameFilter = FilterInterface<
  UnitNameCondition,
  UnitNameOrderBy
>;
