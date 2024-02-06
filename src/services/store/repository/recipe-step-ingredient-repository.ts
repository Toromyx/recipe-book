import type {
  RecipeStepIngredientCreateInterface,
  RecipeStepIngredientInterface,
  RecipeStepIngredientUpdateInterface,
} from "../../../types/entity/recipe-step-ingredient-interface.ts";
import type {
  RecipeStepIngredientCondition,
  RecipeStepIngredientOrderBy,
} from "../../../types/filter/recipe-step-ingredient-filter.ts";
import {
  countRecipeStepIngredient,
  createRecipeStepIngredient,
  deleteRecipeStepIngredient,
  listRecipeStepIngredient,
  readRecipeStepIngredient,
  updateRecipeStepIngredient,
} from "../../command/entity.ts";
import { listen } from "../../event/client.ts";
import { EventChannel } from "../../event/event-channel.ts";
import { EntityRepository } from "./entity-repository.ts";

export const recipeStepIngredientRepository: EntityRepository<
  RecipeStepIngredientInterface,
  RecipeStepIngredientCreateInterface,
  RecipeStepIngredientUpdateInterface,
  RecipeStepIngredientCondition,
  RecipeStepIngredientOrderBy
> = new EntityRepository(
  (entityCreate) => createRecipeStepIngredient(entityCreate),
  (identifier) => readRecipeStepIngredient(identifier),
  (entityUpdate) => updateRecipeStepIngredient(entityUpdate),
  (identifier) => deleteRecipeStepIngredient(identifier),
  (filter) => listRecipeStepIngredient(filter),
  (condition) => countRecipeStepIngredient(condition),
  undefined,
  undefined,
  (reactFunction) => {
    void listen(
      EventChannel.ENTITY_ACTION_UPDATED_RECIPE_STEP_INGREDIENT,
      (event) => {
        reactFunction(event.payload);
      },
    );
  },
  (reactFunction) => {
    void listen(
      EventChannel.ENTITY_ACTION_CREATED_RECIPE_STEP_INGREDIENT,
      () => {
        reactFunction();
      },
    );
  },
  (reactFunction) => {
    void listen(
      EventChannel.ENTITY_ACTION_DELETED_RECIPE_STEP_INGREDIENT,
      (event) => {
        reactFunction(event.payload);
      },
    );
  },
);
