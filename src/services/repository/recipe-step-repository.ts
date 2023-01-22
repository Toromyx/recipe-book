import type { EntityRepositoryInterface } from "./entity-repository.js";
import { EntityRepository } from "./entity-repository.js";
import { apiClient } from "../command/entity.js";
import { client } from "../event/client.js";
import { EventChannel } from "../event/event-channel.js";
import type { RecipeStepInterface } from "../../types/entity/recipe-step-interface.js";
import type { RecipeStepFilterInterface } from "../../types/filter/recipe-step-filter-interface.js";

export const recipeStepRepository: EntityRepositoryInterface<
  RecipeStepInterface,
  RecipeStepFilterInterface
> = new EntityRepository(
  (partial) => apiClient.createRecipeStep(partial),
  (identifier) => apiClient.readRecipeStep(identifier),
  (partial) => apiClient.updateRecipeStep(partial),
  (identifier) => apiClient.deleteRecipeStep(identifier),
  (filter) => apiClient.listRecipeStep(filter),
  (filter) => apiClient.countRecipeStep(filter),
  {},
  (reactFunction) => {
    void client.listen(
      EventChannel.ENTITY_ACTION_UPDATED_RECIPE_STEP,
      (event) => {
        reactFunction(event.payload);
      },
    );
  },
  (reactFunction) => {
    void client.listen(EventChannel.ENTITY_ACTION_CREATED_RECIPE_STEP, () => {
      reactFunction();
    });
  },
  (reactFunction) => {
    void client.listen(
      EventChannel.ENTITY_ACTION_DELETED_RECIPE_STEP,
      (event) => {
        reactFunction(event.payload);
      },
    );
  },
);
