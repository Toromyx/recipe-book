import type { IdentifiableInterface } from "../identifiable-interface.ts";
import type {
  SortableInterface,
  SortableUpdateInterface,
} from "../sortable-interface.ts";

export interface RecipeIngredientDraftInterface
  extends IdentifiableInterface,
    SortableInterface {
  text: string;
  recipeId: number;
}

export interface RecipeIngredientDraftCreateInterface
  extends SortableInterface {
  text: string;
  recipeId: number;
}

export interface RecipeIngredientDraftUpdateInterface
  extends IdentifiableInterface,
    SortableUpdateInterface {
  order?: number;
  text?: string;
}
