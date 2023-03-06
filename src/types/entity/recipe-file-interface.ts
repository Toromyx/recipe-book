import type { IdentifiableInterface } from "../identifiable-interface.ts";

export interface RecipeFileInterface extends IdentifiableInterface {
  name: string;
  order: number;
  mime: string;
  path: string;
  recipeStepId: number;
}

export interface RecipeFileCreateInterface {
  name: string;
  order: number;
  path: string;
  recipeStepId: number;
}

export interface RecipeFileUpdateInterface extends IdentifiableInterface {
  name?: string;
  order?: number;
}
