import type { IdentifiableInterface } from "../identifiable-interface.ts";
import type {
  SortableInterface,
  SortableUpdateInterface,
} from "../sortable-interface.ts";

export interface RecipeStepInterface
  extends IdentifiableInterface,
    SortableInterface {
  description: string;
  recipeId: number;
}

export interface RecipeStepCreateInterface extends SortableInterface {
  description: string;
  recipeId: number;
}

export interface RecipeStepUpdateInterface
  extends IdentifiableInterface,
    SortableUpdateInterface {
  description?: string;
}
