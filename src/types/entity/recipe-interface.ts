import type { IdentifiableInterface } from "../identifiable-interface.ts";

export interface RecipeInterface extends IdentifiableInterface {
  name: string;
}

export interface RecipeCreateInterface {
  name: string;
}

export interface RecipeUpdateInterface extends IdentifiableInterface {
  name?: string;
}
