import type { IdentifiableInterface } from "../identifiable-interface.ts";
import type {
  SortableInterface,
  SortableUpdateInterface,
} from "../sortable-interface.ts";

export interface RecipeIngredientInterface
  extends IdentifiableInterface,
    SortableInterface {
  quantity: number | null;
  unit: string | null;
  recipeStepId: number;
  ingredientId: number;
}

export interface RecipeIngredientCreateInterface extends SortableInterface {
  quantity: number | null;
  unit: string | null;
  recipeStepId: number;
  ingredientId: number;
}

export interface RecipeIngredientUpdateInterface
  extends IdentifiableInterface,
    SortableUpdateInterface {
  order?: number;
  quantity?: number | null;
  unit?: string | null;
  ingredientId?: number;
}
