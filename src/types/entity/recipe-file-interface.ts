import type { IdentifiableInterface } from "../identifiable-interface.ts";
import type {
  SortableInterface,
  SortableUpdateInterface,
} from "../sortable-interface.ts";

export interface RecipeFileInterface
  extends IdentifiableInterface,
    SortableInterface {
  name: string;
  mime: string;
  path: string;
  recipeStepId: number;
}

type RecipeFileCreateUri = { path: string } | { url: string };

export interface RecipeFileCreateInterface extends SortableInterface {
  name: string;
  uri: RecipeFileCreateUri;
  recipeStepId: number;
}

export interface RecipeFileUpdateInterface
  extends IdentifiableInterface,
    SortableUpdateInterface {
  name?: string;
}
