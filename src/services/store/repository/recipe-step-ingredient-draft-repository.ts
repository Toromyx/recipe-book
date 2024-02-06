import type {
  RecipeStepIngredientDraftCreateInterface,
  RecipeStepIngredientDraftInterface,
  RecipeStepIngredientDraftUpdateInterface,
} from "../../../types/entity/recipe-step-ingredient-draft-interface.ts";
import type {
  RecipeStepIngredientDraftCondition,
  RecipeStepIngredientDraftOrderBy,
} from "../../../types/filter/recipe-step-ingredient-draft-filter.ts";
import {
  countRecipeStepIngredientDraft,
  createRecipeStepIngredientDraft,
  deleteRecipeStepIngredientDraft,
  listRecipeStepIngredientDraft,
  readRecipeStepIngredientDraft,
  updateRecipeStepIngredientDraft,
} from "../../command/entity.ts";
import { listen } from "../../event/client.ts";
import { EventChannel } from "../../event/event-channel.ts";
import { EntityRepository } from "./entity-repository.ts";

export const recipeStepIngredientDraftRepository: EntityRepository<
  RecipeStepIngredientDraftInterface,
  RecipeStepIngredientDraftCreateInterface,
  RecipeStepIngredientDraftUpdateInterface,
  RecipeStepIngredientDraftCondition,
  RecipeStepIngredientDraftOrderBy
> = new EntityRepository(
  (entityCreate) => createRecipeStepIngredientDraft(entityCreate),
  (identifier) => readRecipeStepIngredientDraft(identifier),
  (entityUpdate) => updateRecipeStepIngredientDraft(entityUpdate),
  (identifier) => deleteRecipeStepIngredientDraft(identifier),
  (filter) => listRecipeStepIngredientDraft(filter),
  (condition) => countRecipeStepIngredientDraft(condition),
  undefined,
  undefined,
  (reactFunction) => {
    void listen(
      EventChannel.ENTITY_ACTION_UPDATED_RECIPE_STEP_INGREDIENT_DRAFT,
      (event) => {
        reactFunction(event.payload);
      },
    );
  },
  (reactFunction) => {
    void listen(
      EventChannel.ENTITY_ACTION_CREATED_RECIPE_STEP_INGREDIENT_DRAFT,
      () => {
        reactFunction();
      },
    );
  },
  (reactFunction) => {
    void listen(
      EventChannel.ENTITY_ACTION_DELETED_RECIPE_STEP_INGREDIENT_DRAFT,
      (event) => {
        reactFunction(event.payload);
      },
    );
  },
);
