import type {
  RecipeStepCreateInterface,
  RecipeStepInterface,
  RecipeStepUpdateInterface,
} from "../../../types/entity/recipe-step-interface.ts";
import type {
  RecipeStepCondition,
  RecipeStepOrderBy,
} from "../../../types/filter/recipe-step-filter.ts";
import {
  countRecipeStep,
  createRecipeStep,
  deleteRecipeStep,
  listRecipeStep,
  readRecipeStep,
  updateRecipeStep,
} from "../../command/entity.ts";
import { listen } from "../../event/client.ts";
import { EventChannel } from "../../event/event-channel.ts";
import { EntityRepository } from "./entity-repository.ts";

export const recipeStepRepository: EntityRepository<
  RecipeStepInterface,
  RecipeStepCreateInterface,
  RecipeStepUpdateInterface,
  RecipeStepCondition,
  RecipeStepOrderBy
> = new EntityRepository(
  (entityCreate) => createRecipeStep(entityCreate),
  (identifier) => readRecipeStep(identifier),
  (entityUpdate) => updateRecipeStep(entityUpdate),
  (identifier) => deleteRecipeStep(identifier),
  (filter) => listRecipeStep(filter),
  (condition) => countRecipeStep(condition),
  undefined,
  undefined,
  (reactFunction) => {
    void listen(EventChannel.ENTITY_ACTION_UPDATED_RECIPE_STEP, (event) => {
      reactFunction(event.payload);
    });
  },
  (reactFunction) => {
    void listen(EventChannel.ENTITY_ACTION_CREATED_RECIPE_STEP, () => {
      reactFunction();
    });
  },
  (reactFunction) => {
    void listen(EventChannel.ENTITY_ACTION_DELETED_RECIPE_STEP, (event) => {
      reactFunction(event.payload);
    });
  },
);
