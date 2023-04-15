import type {
  IngredientCreateInterface,
  IngredientInterface,
  IngredientUpdateInterface,
} from "../../../types/entity/ingredient-interface.ts";
import type { IngredientFilterInterface } from "../../../types/filter/ingredient-filter-interface.ts";
import {
  countIngredient,
  createIngredient,
  deleteIngredient,
  listIngredient,
  readIngredient,
  updateIngredient,
} from "../../command/entity.ts";
import { listen } from "../../event/client.ts";
import { EventChannel } from "../../event/event-channel.ts";
import { EntityRepository } from "./entity-repository.ts";

export const ingredientRepository: EntityRepository<
  IngredientInterface,
  IngredientCreateInterface,
  IngredientUpdateInterface,
  IngredientFilterInterface
> = new EntityRepository(
  (entityCreate) => createIngredient(entityCreate),
  (identifier) => readIngredient(identifier),
  (entityUpdate) => updateIngredient(entityUpdate),
  (identifier) => deleteIngredient(identifier),
  (filter) => listIngredient(filter),
  (filter) => countIngredient(filter),
  {},
  (reactFunction) => {
    void listen(EventChannel.ENTITY_ACTION_UPDATED_INGREDIENT, (event) => {
      reactFunction(event.payload);
    });
  },
  (reactFunction) => {
    void listen(EventChannel.ENTITY_ACTION_CREATED_INGREDIENT, () => {
      reactFunction();
    });
  },
  (reactFunction) => {
    void listen(EventChannel.ENTITY_ACTION_DELETED_INGREDIENT, (event) => {
      reactFunction(event.payload);
    });
  },
  (reactFunction) => {
    void listen(EventChannel.ENTITY_ACTION_UPDATED_RECIPE_INGREDIENT, () => {
      reactFunction();
    });
    void listen(EventChannel.ENTITY_ACTION_CREATED_RECIPE_INGREDIENT, () => {
      reactFunction();
    });
    void listen(EventChannel.ENTITY_ACTION_DELETED_RECIPE_INGREDIENT, () => {
      reactFunction();
    });
  },
);
