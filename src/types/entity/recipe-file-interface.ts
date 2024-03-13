import type { IdentifiableInterface } from "../identifiable-interface.ts";
import type {
  SortableInterface,
  SortableUpdateInterface,
} from "../sortable-interface.ts";

export interface RecipeFileInterface
  extends IdentifiableInterface,
    SortableInterface {
  recipeId: number;
  fileId: number;
}

export interface RecipeFileCreateInterface extends SortableInterface {
  recipeId: number;
  fileId: number;
}

export interface RecipeFileUpdateInterface
  extends IdentifiableInterface,
    SortableUpdateInterface {}
