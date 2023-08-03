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
    steps: selectMultipleInParentNode(
      recipeDocument,
      '[data-test-id="collapsible-layout"]',
    ).map((stepElement) => ({
      description: readContent(stepElement),
      ingredients: [],
      files: [],
    })),
  };
}

/**
 * @type {ExternalRecipeModule}
 */
export { get };
