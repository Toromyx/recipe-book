import type { IdentifiableInterface } from "../identifiable-interface.ts";

export interface RecipeStepInterface extends IdentifiableInterface {
  order: number;
  description: string;
  image: string | null;
  recipeId: number;
}

export interface RecipeStepCreateInterface {
  order: number;
  description: string;
  image: string | null;
  recipeId: number;
}

export interface RecipeStepUpdateInterface extends IdentifiableInterface {
  order?: number;
  description?: string;
  image?: string | null;
}
