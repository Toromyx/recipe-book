import type {
  IngredientCreateInterface,
  IngredientUpdateInterface,
} from "../../types/entity/ingredient-interface.ts";
import type {
  RecipeFileCreateInterface,
  RecipeFileUpdateInterface,
} from "../../types/entity/recipe-file-interface.ts";
import type {
  RecipeIngredientDraftCreateInterface,
  RecipeIngredientDraftUpdateInterface,
} from "../../types/entity/recipe-ingredient-draft-interface.ts";
import type {
  RecipeIngredientCreateInterface,
  RecipeIngredientUpdateInterface,
} from "../../types/entity/recipe-ingredient-interface.ts";
import type {
  RecipeCreateInterface,
  RecipeUpdateInterface,
} from "../../types/entity/recipe-interface.ts";
import type {
  RecipeStepCreateInterface,
  RecipeStepUpdateInterface,
} from "../../types/entity/recipe-step-interface.ts";
import type {
  Unit,
  UnitNameCreateInterface,
  UnitNameUpdateInterface,
} from "../../types/entity/unit-name-interface.ts";
import type {
  IngredientCondition,
  IngredientFilter,
} from "../../types/filter/ingredient-filter.ts";
import type {
  RecipeFileCondition,
  RecipeFileFilter,
} from "../../types/filter/recipe-file-filter.ts";
import type {
  RecipeCondition,
  RecipeFilter,
} from "../../types/filter/recipe-filter.ts";
import type {
  RecipeIngredientDraftCondition,
  RecipeIngredientDraftFilter,
} from "../../types/filter/recipe-ingredient-draft-filter.ts";
import type {
  RecipeIngredientCondition,
  RecipeIngredientFilter,
} from "../../types/filter/recipe-ingredient-filter.ts";
import type {
  RecipeStepCondition,
  RecipeStepFilter,
} from "../../types/filter/recipe-step-filter.ts";
import type {
  UnitNameCondition,
  UnitNameFilter,
} from "../../types/filter/unit-name-filter.ts";
import type { Command } from "./command.ts";

type CommandParameterMap = {
  [Command.ENTITY_CREATE_INGREDIENT]: { create: IngredientCreateInterface };
  [Command.ENTITY_READ_INGREDIENT]: { id: number };
  [Command.ENTITY_UPDATE_INGREDIENT]: { update: IngredientUpdateInterface };
  [Command.ENTITY_DELETE_INGREDIENT]: { id: number };
  [Command.ENTITY_LIST_INGREDIENT]: { filter: IngredientFilter };
  [Command.ENTITY_COUNT_INGREDIENT]: { condition?: IngredientCondition };

  [Command.ENTITY_CREATE_RECIPE]: { create: RecipeCreateInterface };
  [Command.ENTITY_READ_RECIPE]: { id: number };
  [Command.ENTITY_UPDATE_RECIPE]: { update: RecipeUpdateInterface };
  [Command.ENTITY_DELETE_RECIPE]: { id: number };
  [Command.ENTITY_LIST_RECIPE]: { filter: RecipeFilter };
  [Command.ENTITY_COUNT_RECIPE]: { condition?: RecipeCondition };

  [Command.ENTITY_CREATE_RECIPE_FILE]: {
    create: RecipeFileCreateInterface;
  };
  [Command.ENTITY_READ_RECIPE_FILE]: { id: number };
  [Command.ENTITY_UPDATE_RECIPE_FILE]: {
    update: RecipeFileUpdateInterface;
  };
  [Command.ENTITY_DELETE_RECIPE_FILE]: { id: number };
  [Command.ENTITY_LIST_RECIPE_FILE]: {
    filter: RecipeFileFilter;
  };
  [Command.ENTITY_COUNT_RECIPE_FILE]: {
    condition?: RecipeFileCondition;
  };

  [Command.ENTITY_CREATE_RECIPE_INGREDIENT]: {
    create: RecipeIngredientCreateInterface;
  };
  [Command.ENTITY_READ_RECIPE_INGREDIENT]: { id: number };
  [Command.ENTITY_UPDATE_RECIPE_INGREDIENT]: {
    update: RecipeIngredientUpdateInterface;
  };
  [Command.ENTITY_DELETE_RECIPE_INGREDIENT]: { id: number };
  [Command.ENTITY_LIST_RECIPE_INGREDIENT]: {
    filter: RecipeIngredientFilter;
  };
  [Command.ENTITY_COUNT_RECIPE_INGREDIENT]: {
    condition?: RecipeIngredientCondition;
  };

  [Command.ENTITY_CREATE_RECIPE_INGREDIENT_DRAFT]: {
    create: RecipeIngredientDraftCreateInterface;
  };
  [Command.ENTITY_READ_RECIPE_INGREDIENT_DRAFT]: { id: number };
  [Command.ENTITY_UPDATE_RECIPE_INGREDIENT_DRAFT]: {
    update: RecipeIngredientDraftUpdateInterface;
  };
  [Command.ENTITY_DELETE_RECIPE_INGREDIENT_DRAFT]: { id: number };
  [Command.ENTITY_LIST_RECIPE_INGREDIENT_DRAFT]: {
    filter: RecipeIngredientDraftFilter;
  };
  [Command.ENTITY_COUNT_RECIPE_INGREDIENT_DRAFT]: {
    condition?: RecipeIngredientDraftCondition;
  };

  [Command.ENTITY_CREATE_RECIPE_STEP]: { create: RecipeStepCreateInterface };
  [Command.ENTITY_READ_RECIPE_STEP]: { id: number };
  [Command.ENTITY_UPDATE_RECIPE_STEP]: { update: RecipeStepUpdateInterface };
  [Command.ENTITY_DELETE_RECIPE_STEP]: { id: number };
  [Command.ENTITY_LIST_RECIPE_STEP]: { filter: RecipeStepFilter };
  [Command.ENTITY_COUNT_RECIPE_STEP]: { condition?: RecipeStepCondition };

  [Command.ENTITY_CREATE_UNIT_NAME]: { create: UnitNameCreateInterface };
  [Command.ENTITY_READ_UNIT_NAME]: { id: number };
  [Command.ENTITY_UPDATE_UNIT_NAME]: { update: UnitNameUpdateInterface };
  [Command.ENTITY_DELETE_UNIT_NAME]: { id: number };
  [Command.ENTITY_LIST_UNIT_NAME]: { filter: UnitNameFilter };
  [Command.ENTITY_COUNT_UNIT_NAME]: { condition?: UnitNameCondition };

  [Command.EXTERNAL_RECIPE]: { url: string };

  [Command.OCR]: { recipeFileId: number };

  [Command.UNIT_CONVERT]: { value: number; unit: Unit };

  [Command.UNIT_LIST_GET]: undefined;
};
export type CommandParameter<T extends Command> = CommandParameterMap[T];
