import type {
  RecipeStepFileCreateInterface,
  RecipeStepFileInterface,
  RecipeStepFileUpdateInterface,
} from "../../../types/entity/recipe-step-file-interface.ts";
import type {
  RecipeStepFileCondition,
  RecipeStepFileOrderBy,
} from "../../../types/filter/recipe-step-file-filter.ts";
import {
  countRecipeStepFile,
  createRecipeStepFile,
  deleteRecipeStepFile,
  listRecipeStepFile,
  readRecipeStepFile,
  updateRecipeStepFile,
} from "../../command/entity.ts";
import { listen } from "../../event/client.ts";
import { EventChannel } from "../../event/event-channel.ts";
import { EntityRepository } from "./entity-repository.ts";

export const recipeStepFileRepository: EntityRepository<
  RecipeStepFileInterface,
  RecipeStepFileCreateInterface,
  RecipeStepFileUpdateInterface,
  RecipeStepFileCondition,
  RecipeStepFileOrderBy
> = new EntityRepository(
  (entityCreate) => createRecipeStepFile(entityCreate),
  (identifier) => readRecipeStepFile(identifier),
  (entityUpdate) => updateRecipeStepFile(entityUpdate),
  (identifier) => deleteRecipeStepFile(identifier),
  (filter) => listRecipeStepFile(filter),
  (condition) => countRecipeStepFile(condition),
  undefined,
  undefined,
  (reactFunction) => {
    void listen(
      EventChannel.ENTITY_ACTION_UPDATED_RECIPE_STEP_FILE,
      (event) => {
        reactFunction(event.payload);
      },
    );
  },
  (reactFunction) => {
    void listen(EventChannel.ENTITY_ACTION_CREATED_RECIPE_STEP_FILE, () => {
      reactFunction();
    });
  },
  (reactFunction) => {
    void listen(
      EventChannel.ENTITY_ACTION_DELETED_RECIPE_STEP_FILE,
      (event) => {
        reactFunction(event.payload);
      },
    );
  },
);
