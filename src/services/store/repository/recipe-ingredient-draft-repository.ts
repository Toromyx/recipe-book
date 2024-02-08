import type {
  RecipeIngredientDraftCreateInterface,
  RecipeIngredientDraftInterface,
  RecipeIngredientDraftUpdateInterface,
} from "../../../types/entity/recipe-ingredient-draft-interface.ts";
import type {
  RecipeIngredientDraftCondition,
  RecipeIngredientDraftOrderBy,
} from "../../../types/filter/recipe-ingredient-draft-filter.ts";
import {
  countRecipeIngredientDraft,
  createRecipeIngredientDraft,
  deleteRecipeIngredientDraft,
  listRecipeIngredientDraft,
  readRecipeIngredientDraft,
  updateRecipeIngredientDraft,
} from "../../command/entity.ts";
import { listen } from "../../event/client.ts";
import { EventChannel } from "../../event/event-channel.ts";
import { EntityRepository } from "./entity-repository.ts";

export const recipeIngredientDraftRepository: EntityRepository<
  RecipeIngredientDraftInterface,
  RecipeIngredientDraftCreateInterface,
  RecipeIngredientDraftUpdateInterface,
  RecipeIngredientDraftCondition,
  RecipeIngredientDraftOrderBy
> = new EntityRepository(
  (entityCreate) => createRecipeIngredientDraft(entityCreate),
  (identifier) => readRecipeIngredientDraft(identifier),
  (entityUpdate) => updateRecipeIngredientDraft(entityUpdate),
  (identifier) => deleteRecipeIngredientDraft(identifier),
  (filter) => listRecipeIngredientDraft(filter),
  (condition) => countRecipeIngredientDraft(condition),
  undefined,
  undefined,
  (reactFunction) => {
    void listen(
      EventChannel.ENTITY_ACTION_UPDATED_RECIPE_INGREDIENT_DRAFT,
      (event) => {
        reactFunction(event.payload);
      },
    );
  },
  (reactFunction) => {
    void listen(
      EventChannel.ENTITY_ACTION_CREATED_RECIPE_INGREDIENT_DRAFT,
      () => {
        reactFunction();
      },
    );
  },
  (reactFunction) => {
    void listen(
      EventChannel.ENTITY_ACTION_DELETED_RECIPE_INGREDIENT_DRAFT,
      (event) => {
        reactFunction(event.payload);
      },
    );
  },
);
