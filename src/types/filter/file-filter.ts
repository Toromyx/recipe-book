import type { FilterInterface, Order } from "../filter-interface.ts";

export type FileCondition = {
  name?: string;
};

export type FileOrderBy = { name: Order };

export type FileFilter = FilterInterface<FileCondition, FileOrderBy>;
