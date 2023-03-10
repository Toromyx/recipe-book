import type {
  RecipeIngredientCreateInterface,
  RecipeIngredientInterface,
  RecipeIngredientUpdateInterface,
} from "../../types/entity/recipe-ingredient-interface.ts";
import type { RecipeIngredientFilterInterface } from "../../types/filter/recipe-ingredient-filter-interface.ts";
import { apiClient } from "../command/entity.ts";
import { client } from "../event/client.ts";
import { EventChannel } from "../event/event-channel.ts";
import type { EntityRepositoryInterface } from "./entity-repository.ts";
import { EntityRepository } from "./entity-repository.ts";

export const recipeIngredientRepository: EntityRepositoryInterface<
  RecipeIngredientInterface,
  RecipeIngredientCreateInterface,
  RecipeIngredientUpdateInterface,
  RecipeIngredientFilterInterface
> = new EntityRepository(
  (entityCreate) => apiClient.createRecipeIngredient(entityCreate),
  (identifier) => apiClient.readRecipeIngredient(identifier),
  (entityUpdate) => apiClient.updateRecipeIngredient(entityUpdate),
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
