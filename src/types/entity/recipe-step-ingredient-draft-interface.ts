import type { IdentifiableInterface } from "../identifiable-interface.ts";
import type {
  SortableInterface,
  SortableUpdateInterface,
} from "../sortable-interface.ts";

export interface RecipeStepIngredientDraftInterface
  extends IdentifiableInterface,
    SortableInterface {
  text: string;
  recipeStepId: number;
}

export interface RecipeStepIngredientDraftCreateInterface
  extends SortableInterface {
  text: string;
  recipeStepId: number;
}

export interface RecipeStepIngredientDraftUpdateInterface
  extends IdentifiableInterface,
    SortableUpdateInterface {
  order?: number;
  text?: string;
}
