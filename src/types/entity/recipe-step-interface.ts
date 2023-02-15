import type { IdentifiableInterface } from "../identifiable-interface.ts";

export interface RecipeStepInterface extends IdentifiableInterface {
  order: number;
  description: string;
  recipeId: number;
}
