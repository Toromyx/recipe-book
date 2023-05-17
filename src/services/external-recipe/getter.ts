import type { ExternalRecipe, ExternalRecipeData } from "../external-recipe.ts";

export type ExternalRecipeModule = {
  get: (data: string) => ExternalRecipe;
};

const modules = import.meta.glob("./modules/*.ts") as Record<
  string,
  () => Promise<ExternalRecipeModule>
>;

const getModulePath = (name: string) => `./modules/${name}.ts`;

const getModule = (name: string): Promise<ExternalRecipeModule> =>
  modules[getModulePath(name)]();

export async function getExternalRecipe(
  externalRecipeData: ExternalRecipeData,
): Promise<ExternalRecipe> {
  if ("jsModule" in externalRecipeData.instructions) {
    const module = await getModule(
      externalRecipeData.instructions.jsModule.name,
    );
    return module.get(externalRecipeData.data);
  }

  throw new Error("");
}

/**
 * Select a single element in the parent node.
 * @throws {Error} when the element is not found.
 */
export function selectInParentNode(
  parentNode: ParentNode,
  selector: string,
): HTMLElement {
  const element = parentNode.querySelector(selector);
  if (!element) {
    throw new Error(
      `Could not select an element in a node with selector "${selector}".`,
    );
  }

  return element as HTMLElement;
}

/**
 * Select multiple elements in the parent node.
 */
export function selectMultipleInParentNode(
  parentNode: ParentNode,
  selector: string,
): HTMLElement[] {
  const elements = parentNode.querySelectorAll(selector);
  return [...elements] as HTMLElement[];
}

export function readContent(element: HTMLElement): string {
  return element.innerText;
}
