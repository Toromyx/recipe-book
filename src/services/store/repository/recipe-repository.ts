import type {
  RecipeCreateInterface,
  RecipeInterface,
  RecipeUpdateInterface,
} from "../../../types/entity/recipe-interface.ts";
import type { RecipeFilterInterface } from "../../../types/filter/recipe-filter-interface.ts";
import {
  countRecipe,
  createRecipe,
  deleteRecipe,
  listRecipe,
  readRecipe,
  updateRecipe,
} from "../../command/entity.ts";
import { listen } from "../../event/client.ts";
import { EventChannel } from "../../event/event-channel.ts";
import { EntityRepository } from "./entity-repository.ts";

export const recipeRepository: EntityRepository<
  RecipeInterface,
  RecipeCreateInterface,
  RecipeUpdateInterface,
  RecipeFilterInterface
> = new EntityRepository(
  (entityCreate) => createRecipe(entityCreate),
  (identifier) => readRecipe(identifier),
  (entityUpdate) => updateRecipe(entityUpdate),
  (identifier) => deleteRecipe(identifier),
  (filter) => listRecipe(filter),
  (filter) => countRecipe(filter),
  {},
  (reactFunction) => {
    void listen(EventChannel.ENTITY_ACTION_UPDATED_RECIPE, (event) => {
      reactFunction(event.payload);
    });
  },
  (reactFunction) => {
    void listen(EventChannel.ENTITY_ACTION_CREATED_RECIPE, () => {
      reactFunction();
    });
  },
  (reactFunction) => {
    void listen(EventChannel.ENTITY_ACTION_DELETED_RECIPE, (event) => {
      reactFunction(event.payload);
    });
  },
);
