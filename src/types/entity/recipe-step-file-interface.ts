import type { IdentifiableInterface } from "../identifiable-interface.ts";
import type {
  SortableInterface,
  SortableUpdateInterface,
} from "../sortable-interface.ts";

export interface RecipeStepFileInterface
  extends IdentifiableInterface,
    SortableInterface {
  name: string;
  mime: string;
  path: string;
  recipeStepId: number;
}

type RecipeStepFileCreateUri = { path: string } | { url: string };

export interface RecipeStepFileCreateInterface extends SortableInterface {
  name: string;
  uri: RecipeStepFileCreateUri;
  recipeStepId: number;
}

export interface RecipeStepFileUpdateInterface
  extends IdentifiableInterface,
    SortableUpdateInterface {
  name?: string;
}
