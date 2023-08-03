import type { ExternalRecipe } from "../../external-recipe.ts";
import {
  readContent,
  selectInParentNode,
  selectMultipleInParentNode,
} from "../getter.ts";

function get(data: string): ExternalRecipe {
  const parser = new DOMParser();
  const recipeDocument = parser.parseFromString(data, "text/html");

  return {
    name: readContent(selectInParentNode(recipeDocument, "h1")),
    steps: selectMultipleInParentNode(recipeDocument, ".recipe").map(
      (stepElement) => ({
        description: readContent(stepElement),
        ingredients: selectMultipleInParentNode(
          recipeDocument,
          ".flex.items-start.justify-start.mb-1.space-x-3.text-lg.sm\\:text-base",
        ).map((ingredientElement) => readContent(ingredientElement).trim()),
        files: [],
      }),
    ),
  };
}

/**
 * @type {ExternalRecipeModule}
 */
export { get };
