import type { EntityRepositoryInterface } from "./entity-repository.ts";
import { EntityRepository } from "./entity-repository.ts";
import { apiClient } from "../command/entity.ts";
import { client } from "../event/client.ts";
import { EventChannel } from "../event/event-channel.ts";
import type { RecipeIngredientInterface } from "../../types/entity/recipe-ingredient-interface.ts";
import type { RecipeIngredientFilterInterface } from "../../types/filter/recipe-ingredient-filter-interface.ts";

export const recipeIngredientRepository: EntityRepositoryInterface<
  RecipeIngredientInterface,
  RecipeIngredientFilterInterface
> = new EntityRepository(
  (partial) => apiClient.createRecipeIngredient(partial),
  (identifier) => apiClient.readRecipeIngredient(identifier),
  (partial) => apiClient.updateRecipeIngredient(partial),
  (identifier) => apiClient.deleteRecipeIngredient(identifier),
  (filter) => apiClient.listRecipeIngredient(filter),
  (filter) => apiClient.countRecipeIngredient(filter),
  {},
  (reactFunction) => {
    void client.listen(
      EventChannel.ENTITY_ACTION_UPDATED_RECIPE_INGREDIENT,
      (event) => {
        reactFunction(event.payload);
      },
    );
  },
  (reactFunction) => {
    void client.listen(
      EventChannel.ENTITY_ACTION_CREATED_RECIPE_INGREDIENT,
      () => {
        reactFunction();
      },
    );
  },
  (reactFunction) => {
    void client.listen(
      EventChannel.ENTITY_ACTION_DELETED_RECIPE_INGREDIENT,
      (event) => {
        reactFunction(event.payload);
      },
    );
  },
);
