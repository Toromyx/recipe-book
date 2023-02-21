import type { IdentifiableInterface } from "../identifiable-interface.ts";

export interface RecipeIngredientInterface extends IdentifiableInterface {
  order: number;
  quantity: number | null;
  unit: string | null;
  recipeStepId: number;
  ingredientId: number;
}

export interface RecipeIngredientCreateInterface {
  order: number;
  quantity: number | null;
  unit: string | null;
  recipeStepId: number;
  ingredientId: number;
}

export interface RecipeIngredientUpdateInterface extends IdentifiableInterface {
  order?: number;
  quantity?: number | null;
  unit?: string | null;
  ingredientId?: number;
}
