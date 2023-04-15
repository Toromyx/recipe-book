import type {
  RecipeIngredientCreateInterface,
  RecipeIngredientInterface,
  RecipeIngredientUpdateInterface,
} from "../../../types/entity/recipe-ingredient-interface.ts";
import type { RecipeIngredientFilterInterface } from "../../../types/filter/recipe-ingredient-filter-interface.ts";
import {
  countRecipeIngredient,
  createRecipeIngredient,
  deleteRecipeIngredient,
  listRecipeIngredient,
  readRecipeIngredient,
  updateRecipeIngredient,
} from "../../command/entity.ts";
import { listen } from "../../event/client.ts";
import { EventChannel } from "../../event/event-channel.ts";
import { EntityRepository } from "./entity-repository.ts";

export const recipeIngredientRepository: EntityRepository<
  RecipeIngredientInterface,
  RecipeIngredientCreateInterface,
  RecipeIngredientUpdateInterface,
  RecipeIngredientFilterInterface
> = new EntityRepository(
  (entityCreate) => createRecipeIngredient(entityCreate),
  (identifier) => readRecipeIngredient(identifier),
  (entityUpdate) => updateRecipeIngredient(entityUpdate),
  (identifier) => deleteRecipeIngredient(identifier),
  (filter) => listRecipeIngredient(filter),
  (filter) => countRecipeIngredient(filter),
  {},
  (reactFunction) => {
    void listen(
      EventChannel.ENTITY_ACTION_UPDATED_RECIPE_INGREDIENT,
      (event) => {
        reactFunction(event.payload);
      },
    );
  },
  (reactFunction) => {
    void listen(EventChannel.ENTITY_ACTION_CREATED_RECIPE_INGREDIENT, () => {
      reactFunction();
    });
  },
  (reactFunction) => {
    void listen(
      EventChannel.ENTITY_ACTION_DELETED_RECIPE_INGREDIENT,
      (event) => {
        reactFunction(event.payload);
      },
    );
  },
);
