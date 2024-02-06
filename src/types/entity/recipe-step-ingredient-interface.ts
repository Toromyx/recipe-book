import type { IdentifiableInterface } from "../identifiable-interface.ts";
import type {
  SortableInterface,
  SortableUpdateInterface,
} from "../sortable-interface.ts";

export interface RecipeStepIngredientInterface
  extends IdentifiableInterface,
    SortableInterface {
  quantity: number | null;
  unit: string | null;
  quality: string | null;
  recipeStepId: number;
  ingredientId: number;
}

export interface RecipeStepIngredientCreateInterface extends SortableInterface {
  quantity: number | null;
  unit: string | null;
  quality: string | null;
  recipeStepId: number;
  ingredientId: number;
}

export interface RecipeStepIngredientUpdateInterface
  extends IdentifiableInterface,
    SortableUpdateInterface {
  order?: number;
  quantity?: number | null;
  unit?: string | null;
  quality?: string | null;
  ingredientId?: number;
}
