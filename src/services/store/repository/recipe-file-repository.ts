import type {
  RecipeFileCreateInterface,
  RecipeFileInterface,
  RecipeFileUpdateInterface,
} from "../../../types/entity/recipe-file-interface.ts";
import type { RecipeFileFilterInterface } from "../../../types/filter/recipe-file-filter-interface.ts";
import { apiClient } from "../../command/entity.ts";
import { client } from "../../event/client.ts";
import { EventChannel } from "../../event/event-channel.ts";
import type { EntityRepositoryInterface } from "./entity-repository.ts";
import { EntityRepository } from "./entity-repository.ts";

export const recipeFileRepository: EntityRepositoryInterface<
  RecipeFileInterface,
  RecipeFileCreateInterface,
  RecipeFileUpdateInterface,
  RecipeFileFilterInterface
> = new EntityRepository(
  (entityCreate) => apiClient.createRecipeFile(entityCreate),
  (identifier) => apiClient.readRecipeFile(identifier),
  (entityUpdate) => apiClient.updateRecipeFile(entityUpdate),
  (identifier) => apiClient.deleteRecipeFile(identifier),
  (filter) => apiClient.listRecipeFile(filter),
  (filter) => apiClient.countRecipeFile(filter),
  {},
  (reactFunction) => {
    void client.listen(
      EventChannel.ENTITY_ACTION_UPDATED_RECIPE_FILE,
      (event) => {
        reactFunction(event.payload);
      },
    );
  },
  (reactFunction) => {
    void client.listen(EventChannel.ENTITY_ACTION_CREATED_RECIPE_FILE, () => {
      reactFunction();
    });
  },
  (reactFunction) => {
    void client.listen(
      EventChannel.ENTITY_ACTION_DELETED_RECIPE_FILE,
      (event) => {
        reactFunction(event.payload);
      },
    );
  },
);
