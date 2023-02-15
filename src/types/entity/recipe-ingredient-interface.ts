import type { IdentifiableInterface } from "../identifiable-interface.ts";

export interface RecipeIngredientInterface extends IdentifiableInterface {
  order: number;
  quantity: number;
  unit: string;
  recipeStepId: number;
  ingredientId: number;
}
