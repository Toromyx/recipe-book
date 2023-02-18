import type { IdentifiableInterface } from "../identifiable-interface.ts";

export interface IngredientInterface extends IdentifiableInterface {
  name: string;
}

export interface IngredientCreateInterface {
  name: string;
}

export interface IngredientUpdateInterface extends IdentifiableInterface {
  name?: string;
}
