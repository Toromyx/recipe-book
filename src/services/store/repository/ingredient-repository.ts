import type {
  IngredientCreateInterface,
  IngredientInterface,
  IngredientUpdateInterface,
} from "../../../types/entity/ingredient-interface.ts";
import type {
  IngredientCondition,
  IngredientOrderBy,
} from "../../../types/filter/ingredient-filter.ts";
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
  IngredientCondition,
  IngredientOrderBy
> = new EntityRepository(
  (entityCreate) => createIngredient(entityCreate),
  (identifier) => readIngredient(identifier),
  (entityUpdate) => updateIngredient(entityUpdate),
  (identifier) => deleteIngredient(identifier),
  (filter) => listIngredient(filter),
  (condition) => countIngredient(condition),
  undefined,
  undefined,
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
    void listen(
      EventChannel.ENTITY_ACTION_UPDATED_RECIPE_STEP_INGREDIENT,
      () => {
        reactFunction();
      },
    );
    void listen(
      EventChannel.ENTITY_ACTION_CREATED_RECIPE_STEP_INGREDIENT,
      () => {
        reactFunction();
      },
    );
    void listen(
      EventChannel.ENTITY_ACTION_DELETED_RECIPE_STEP_INGREDIENT,
      () => {
        reactFunction();
      },
    );
  },
);
