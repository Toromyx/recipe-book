import type { IdentifiableInterface } from "../identifiable-interface.ts";

export interface RecipeIngredientInterface extends IdentifiableInterface {
  order: number;
  quantity: number;
  unit: string;
  recipeStepId: number;
  ingredientId: number;
}

export interface RecipeIngredientCreateInterface {
  order: number;
  quantity: number;
  unit: string;
  recipeStepId: number;
  ingredientId: number;
}

export interface RecipeIngredientUpdateInterface extends IdentifiableInterface {
  order?: number;
  quantity?: number;
  unit?: string;
  ingredientId?: number;
}
