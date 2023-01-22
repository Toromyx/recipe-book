import type { IdentifiableInterface } from "../identifiable-interface.js";

export interface RecipeStepInterface extends IdentifiableInterface {
  order: number;
  description: string;
  recipeId: number;
}
