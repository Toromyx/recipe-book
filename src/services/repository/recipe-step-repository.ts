import type {
  RecipeStepCreateInterface,
  RecipeStepInterface,
  RecipeStepUpdateInterface,
} from "../../types/entity/recipe-step-interface.ts";
import type { RecipeStepFilterInterface } from "../../types/filter/recipe-step-filter-interface.ts";
import { apiClient } from "../command/entity.ts";
import { client } from "../event/client.ts";
import { EventChannel } from "../event/event-channel.ts";
import type { EntityRepositoryInterface } from "./entity-repository.ts";
import { EntityRepository } from "./entity-repository.ts";

export const recipeStepRepository: EntityRepositoryInterface<
  RecipeStepInterface,
  RecipeStepCreateInterface,
  RecipeStepUpdateInterface,
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
