import { IdentifiableInterface } from "../identifiable-interface.js";

export interface RecipeIngredientInterface extends IdentifiableInterface {
  order: number;
  quantity: number;
  unit: string;
  recipeStepId: number;
  ingredientId: number;
}
