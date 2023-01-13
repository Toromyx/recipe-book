export interface FilterInterface<Condition, OrderBy> {
  condition?: Condition;
  orderBy?: Array<{
    column: OrderBy;
    order?: "asc" | "desc";
  }>;
}
