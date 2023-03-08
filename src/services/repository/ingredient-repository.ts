import type {
  IngredientCreateInterface,
  IngredientInterface,
  IngredientUpdateInterface,
} from "../../types/entity/ingredient-interface.ts";
import type { IngredientFilterInterface } from "../../types/filter/ingredient-filter-interface.ts";
import { apiClient } from "../command/entity.ts";
import { client } from "../event/client.ts";
import { EventChannel } from "../event/event-channel.ts";
import { EntityRepository } from "./entity-repository.ts";
import type { EntityRepositoryInterface } from "./entity-repository.ts";

export const ingredientRepository: EntityRepositoryInterface<
  IngredientInterface,
  IngredientCreateInterface,
  IngredientUpdateInterface,
  IngredientFilterInterface
> = new EntityRepository(
  (entityCreate) => apiClient.createIngredient(entityCreate),
  (identifier) => apiClient.readIngredient(identifier),
  (entityUpdate) => apiClient.updateIngredient(entityUpdate),
  (identifier) => apiClient.deleteIngredient(identifier),
  (filter) => apiClient.listIngredient(filter),
  (filter) => apiClient.countIngredient(filter),
  {},
  (reactFunction) => {
    void client.listen(
      EventChannel.ENTITY_ACTION_UPDATED_INGREDIENT,
      (event) => {
        reactFunction(event.payload);
      },
    );
  },
  (reactFunction) => {
    void client.listen(EventChannel.ENTITY_ACTION_CREATED_INGREDIENT, () => {
      reactFunction();
    });
  },
  (reactFunction) => {
    void client.listen(
      EventChannel.ENTITY_ACTION_DELETED_INGREDIENT,
      (event) => {
        reactFunction(event.payload);
      },
    );
  },
  (reactFunction) => {
    void client.listen(
      EventChannel.ENTITY_ACTION_UPDATED_RECIPE_INGREDIENT,
      () => {
        reactFunction();
      },
    );
    void client.listen(
      EventChannel.ENTITY_ACTION_CREATED_RECIPE_INGREDIENT,
      () => {
        reactFunction();
      },
    );
    void client.listen(
      EventChannel.ENTITY_ACTION_DELETED_RECIPE_INGREDIENT,
      () => {
        reactFunction();
      },
    );
  },
);
