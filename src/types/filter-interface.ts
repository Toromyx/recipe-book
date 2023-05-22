export interface FilterInterface<Condition, OrderBy> {
  condition?: Condition;
  orderBy?: Array<OrderBy>;
}

export type Order = "asc" | "desc";
