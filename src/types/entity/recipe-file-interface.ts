import type { IdentifiableInterface } from "../identifiable-interface.ts";
import type {
  SortableInterface,
  SortableUpdateInterface,
} from "../sortable-interface.ts";

export interface RecipeFileInterface
  extends IdentifiableInterface,
    SortableInterface {
  name: string;
  mime: string;
  path: string;
  recipeStepId: number;
}

export interface RecipeFileCreateInterface extends SortableInterface {
  name: string;
  path: string;
  recipeStepId: number;
}

export interface RecipeFileUpdateInterface
  extends IdentifiableInterface,
    SortableUpdateInterface {
  name?: string;
}
