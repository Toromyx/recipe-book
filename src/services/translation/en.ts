import type { TranslationStrings } from "./translations.ts";
import { constructMessageProxy } from "./translations.ts";

const translationStrings: TranslationStrings = {
  labels: {
    actions: {
      create: "Create",
      edit: "Edit",
      update: "Update",
      delete: "Delete",
      cancel: "Cancel",
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
        image: "Image",
      },
    },
  },
  validity: {
    autocomplete: {
      max: "At most {max} {max,plural,one{element needs}other{elements need}} to be selected.",
      min: "At least {min} {min,plural,one{element needs}other{elements need}} to be selected.",
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
