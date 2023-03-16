import type {
  RecipeCreateInterface,
  RecipeInterface,
  RecipeUpdateInterface,
} from "../../../types/entity/recipe-interface.ts";
import type { RecipeFilterInterface } from "../../../types/filter/recipe-filter-interface.ts";
import { apiClient } from "../../command/entity.ts";
import { client } from "../../event/client.ts";
import { EventChannel } from "../../event/event-channel.ts";
import type { EntityRepositoryInterface } from "./entity-repository.ts";
import { EntityRepository } from "./entity-repository.ts";

export const recipeRepository: EntityRepositoryInterface<
  RecipeInterface,
  RecipeCreateInterface,
  RecipeUpdateInterface,
  RecipeFilterInterface
> = new EntityRepository(
  (entityCreate) => apiClient.createRecipe(entityCreate),
  (identifier) => apiClient.readRecipe(identifier),
  (entityUpdate) => apiClient.updateRecipe(entityUpdate),
  (identifier) => apiClient.deleteRecipe(identifier),
  (filter) => apiClient.listRecipe(filter),
  (filter) => apiClient.countRecipe(filter),
  {},
  (reactFunction) => {
    void client.listen(EventChannel.ENTITY_ACTION_UPDATED_RECIPE, (event) => {
      reactFunction(event.payload);
    });
  },
  (reactFunction) => {
    void client.listen(EventChannel.ENTITY_ACTION_CREATED_RECIPE, () => {
      reactFunction();
    });
  },
  (reactFunction) => {
    void client.listen(EventChannel.ENTITY_ACTION_DELETED_RECIPE, (event) => {
      reactFunction(event.payload);
    });
  },
);
