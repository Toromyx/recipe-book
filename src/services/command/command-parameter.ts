import type {
  FileCreateInterface,
  FileUpdateInterface,
} from "../../types/entity/file-interface.ts";
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
  RecipeCreateInterface,
  RecipeUpdateInterface,
} from "../../types/entity/recipe-interface.ts";
import type {
  RecipeStepFileCreateInterface,
  RecipeStepFileUpdateInterface,
} from "../../types/entity/recipe-step-file-interface.ts";
import type {
  RecipeStepIngredientDraftCreateInterface,
  RecipeStepIngredientDraftUpdateInterface,
} from "../../types/entity/recipe-step-ingredient-draft-interface.ts";
import type {
  RecipeStepIngredientCreateInterface,
  RecipeStepIngredientUpdateInterface,
} from "../../types/entity/recipe-step-ingredient-interface.ts";
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
  FileCondition,
  FileFilter,
} from "../../types/filter/file-filter.ts";
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
  RecipeStepFileCondition,
  RecipeStepFileFilter,
} from "../../types/filter/recipe-step-file-filter.ts";
import type {
  RecipeStepCondition,
  RecipeStepFilter,
} from "../../types/filter/recipe-step-filter.ts";
import type {
  RecipeStepIngredientDraftCondition,
  RecipeStepIngredientDraftFilter,
} from "../../types/filter/recipe-step-ingredient-draft-filter.ts";
import type {
  RecipeStepIngredientCondition,
  RecipeStepIngredientFilter,
} from "../../types/filter/recipe-step-ingredient-filter.ts";
import type {
  UnitNameCondition,
  UnitNameFilter,
} from "../../types/filter/unit-name-filter.ts";
import type { Command } from "./command.ts";

type CommandParameterMap = {
  [Command.ENTITY_CREATE_FILE]: { create: FileCreateInterface };
  [Command.ENTITY_READ_FILE]: { id: number };
  [Command.ENTITY_UPDATE_FILE]: { update: FileUpdateInterface };
  [Command.ENTITY_DELETE_FILE]: { id: number };
  [Command.ENTITY_LIST_FILE]: { filter: FileFilter };
  [Command.ENTITY_COUNT_FILE]: { condition?: FileCondition };

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

  [Command.ENTITY_CREATE_RECIPE_STEP_FILE]: {
    create: RecipeStepFileCreateInterface;
  };
  [Command.ENTITY_READ_RECIPE_STEP_FILE]: { id: number };
  [Command.ENTITY_UPDATE_RECIPE_STEP_FILE]: {
    update: RecipeStepFileUpdateInterface;
  };
  [Command.ENTITY_DELETE_RECIPE_STEP_FILE]: { id: number };
  [Command.ENTITY_LIST_RECIPE_STEP_FILE]: {
    filter: RecipeStepFileFilter;
  };
  [Command.ENTITY_COUNT_RECIPE_STEP_FILE]: {
    condition?: RecipeStepFileCondition;
  };

  [Command.ENTITY_CREATE_RECIPE_STEP_INGREDIENT]: {
    create: RecipeStepIngredientCreateInterface;
  };
  [Command.ENTITY_READ_RECIPE_STEP_INGREDIENT]: { id: number };
  [Command.ENTITY_UPDATE_RECIPE_STEP_INGREDIENT]: {
    update: RecipeStepIngredientUpdateInterface;
  };
  [Command.ENTITY_DELETE_RECIPE_STEP_INGREDIENT]: { id: number };
  [Command.ENTITY_LIST_RECIPE_STEP_INGREDIENT]: {
    filter: RecipeStepIngredientFilter;
  };
  [Command.ENTITY_COUNT_RECIPE_STEP_INGREDIENT]: {
    condition?: RecipeStepIngredientCondition;
  };

  [Command.ENTITY_CREATE_RECIPE_STEP_INGREDIENT_DRAFT]: {
    create: RecipeStepIngredientDraftCreateInterface;
  };
  [Command.ENTITY_READ_RECIPE_STEP_INGREDIENT_DRAFT]: { id: number };
  [Command.ENTITY_UPDATE_RECIPE_STEP_INGREDIENT_DRAFT]: {
    update: RecipeStepIngredientDraftUpdateInterface;
  };
  [Command.ENTITY_DELETE_RECIPE_STEP_INGREDIENT_DRAFT]: { id: number };
  [Command.ENTITY_LIST_RECIPE_STEP_INGREDIENT_DRAFT]: {
    filter: RecipeStepIngredientDraftFilter;
  };
  [Command.ENTITY_COUNT_RECIPE_STEP_INGREDIENT_DRAFT]: {
    condition?: RecipeStepIngredientDraftCondition;
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

  [Command.OCR]: { fileId: number };

  [Command.UNIT_CONVERT]: { value: number; unit: Unit };

  [Command.UNIT_LIST_GET]: undefined;
};
export type CommandParameter<T extends Command> = CommandParameterMap[T];
