import {
  EntityRepository,
  EntityRepositoryInterface,
} from "./entity-repository.js";
import { apiClient } from "../command/entity.js";
import { client } from "../event/client.js";
import { EventChannel } from "../event/event-channel.js";
import { RecipeIngredientInterface } from "../../types/entity/recipe-ingredient-interface.js";
import { RecipeIngredientFilterInterface } from "../../types/filter/recipe-ingredient-filter-interface.js";

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
