import type { RouteDefinition } from "svelte-spa-router";
import RecipeListScreen from "../components/screens/RecipeListScreen.svelte";
import RecipeScreen from "../components/screens/RecipeScreen.svelte";

function maybeEscape<T extends boolean | number | string>(
  component: T,
  doEscape: boolean,
): string | T {
  return doEscape ? encodeURIComponent(component) : component;
}

export function recipeListRoute() {
  return "/";
}
export function recipeRoute(id: number | string, doEscape = true) {
  return `/recipe/${maybeEscape(id, doEscape)}`;
}

/**
 * This object contains all the app's routes for constructing the router.
 */
export const routes: RouteDefinition = {
  [recipeListRoute()]: RecipeListScreen,
  [recipeRoute(":id", false)]: RecipeScreen,
};
