import type { EntityRepositoryInterface } from "./entity-repository.js";
import { EntityRepository } from "./entity-repository.js";
import { apiClient } from "../command/entity.js";
import { client } from "../event/client.js";
import { EventChannel } from "../event/event-channel.js";
import type { RecipeInterface } from "../../types/entity/recipe-interface.js";
import type { RecipeFilterInterface } from "../../types/filter/recipe-filter-interface.js";

export const recipeRepository: EntityRepositoryInterface<
  RecipeInterface,
  RecipeFilterInterface
> = new EntityRepository(
  (partial) => apiClient.createRecipe(partial),
  (identifier) => apiClient.readRecipe(identifier),
  (partial) => apiClient.updateRecipe(partial),
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
