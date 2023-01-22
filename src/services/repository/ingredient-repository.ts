import type { EntityRepositoryInterface } from "./entity-repository.js";
import { EntityRepository } from "./entity-repository.js";
import { apiClient } from "../command/entity.js";
import type { IngredientInterface } from "../../types/entity/ingredient-interface.js";
import type { IngredientFilterInterface } from "../../types/filter/ingredient-filter-interface.js";
import { client } from "../event/client.js";
import { EventChannel } from "../event/event-channel.js";

export const ingredientRepository: EntityRepositoryInterface<
  IngredientInterface,
  IngredientFilterInterface
> = new EntityRepository(
  (partial) => apiClient.createIngredient(partial),
  (identifier) => apiClient.readIngredient(identifier),
  (partial) => apiClient.updateIngredient(partial),
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
);
