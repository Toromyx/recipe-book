import type { IdentifiableInterface } from "../identifiable-interface.ts";
import type {
  SortableInterface,
  SortableUpdateInterface,
} from "../sortable-interface.ts";

export interface RecipeStepFileInterface
  extends IdentifiableInterface,
    SortableInterface {
  recipeStepId: number;
  fileId: number;
}

export interface RecipeStepFileCreateInterface extends SortableInterface {
  recipeStepId: number;
  fileId: number;
}

export interface RecipeStepFileUpdateInterface
  extends IdentifiableInterface,
    SortableUpdateInterface {}
