import type { TranslationStrings } from "./translations.ts";
import { constructMessageProxy } from "./translations.ts";

const translationStrings: TranslationStrings = {
  labels: {
    actions: {
      create: "Create",
      update: "Update",
      delete: "Delete",
    },
    entityFields: {
      ingredient: {
        name: "Ingredient Name",
      },
      recipe: {
        name: "Recipe Name",
      },
      recipeIngredient: {
        quantity: "Quantity",
        unit: "Unit",
        ingredient: "Ingredient",
      },
      recipeStep: {
        description: "Description",
      },
    },
  },
  imperatives: {
    selectPlaceholder: "Select {label}",
  },
  headings: {
    ingredients: "Ingredients",
    recipeStep: "Step {number}",
  },
};

export const messages = constructMessageProxy(translationStrings);
