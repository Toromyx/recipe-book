import type {
  RecipeFileCreateInterface,
  RecipeFileInterface,
  RecipeFileUpdateInterface,
} from "../../../types/entity/recipe-file-interface.ts";
import type { RecipeFileFilterInterface } from "../../../types/filter/recipe-file-filter-interface.ts";
import {
  countRecipeFile,
  createRecipeFile,
  deleteRecipeFile,
  listRecipeFile,
  readRecipeFile,
  updateRecipeFile,
} from "../../command/entity.ts";
import { listen } from "../../event/client.ts";
import { EventChannel } from "../../event/event-channel.ts";
import { EntityRepository } from "./entity-repository.ts";

export const recipeFileRepository: EntityRepository<
  RecipeFileInterface,
  RecipeFileCreateInterface,
  RecipeFileUpdateInterface,
  RecipeFileFilterInterface
> = new EntityRepository(
  (entityCreate) => createRecipeFile(entityCreate),
  (identifier) => readRecipeFile(identifier),
  (entityUpdate) => updateRecipeFile(entityUpdate),
  (identifier) => deleteRecipeFile(identifier),
  (filter) => listRecipeFile(filter),
  (filter) => countRecipeFile(filter),
  {},
  (reactFunction) => {
    void listen(EventChannel.ENTITY_ACTION_UPDATED_RECIPE_FILE, (event) => {
      reactFunction(event.payload);
    });
  },
  (reactFunction) => {
    void listen(EventChannel.ENTITY_ACTION_CREATED_RECIPE_FILE, () => {
      reactFunction();
    });
  },
  (reactFunction) => {
    void listen(EventChannel.ENTITY_ACTION_DELETED_RECIPE_FILE, (event) => {
      reactFunction(event.payload);
    });
  },
);
