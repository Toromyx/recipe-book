import type { TranslationStrings } from "./translations.ts";
import { constructMessageProxy } from "./translations.ts";

const translationStrings: TranslationStrings = {
  labels: {
    actions: {
      create: "Create",
      edit: "Edit",
      update: "Update",
      delete: "Delete",
      deleteSelectedItems: "Delete selected items",
      cancel: "Cancel",
      confirm: "Confirm",
      add: "Add",
      remove: "Remove",
      file: {
        open: "Open",
      },
      ocr: "OCR",
    },
    entityFields: {
      ingredient: {
        name: "Ingredient Name",
      },
      recipe: {
        name: "Recipe Name",
      },
      recipeFile: {
        name: "Name",
        path: "File",
      },
      recipeIngredient: {
        quantity: "Quantity",
        unit: "Unit",
        ingredient: "Ingredient",
        quality: "Quality",
      },
      recipeStep: {
        description: "Description",
      },
    },
    descriptions: {
      ocrOutput: "OCR output",
      progress: {
        loadingExternalRecipe: "Loading external recipe",
      },
      bulkActions: {
        selectItem: "Select this item for bulk actions",
        selectAllItems: "Select all items for bulk actions",
      },
    },
    factor: "Factor",
  },
  validity: {
    autocomplete: {
      max: `.match {$max :number}
        1 {{At most {$max} element needs to be selected.}}
        * {{At most {$max} elements need to be selected.}}`,
      min: `.match {$min :number}
        1 {{At least {$min} element needs to be selected.}}
        * {{At least {$min} elements need to be selected.}}`,
      includesExcluded: "The current selection includes an excluded value.",
    },
    externalRecipeUrlNotSupported:
      'The external recipe URL "{$url}" is not supported.',
  },
  questions: {
    confirmation: "Are you sure?",
  },
  imperatives: {
    selectPlaceholder: "Select {$label}",
  },
  headings: {
    recipeStep: "Step {$number}",
    ingredients: "Ingredients",
    description: "Description",
    files: "Files",
  },
  units: {
    kilogram: "kg",
    gram: "g",
    pound: "lb",
    litre: "l",
    millilitre: "ml",
    usCup: ".match {$value :number} 1 {{cup}} * {{cups}}",
  },
};

export const messages = constructMessageProxy(translationStrings);
