import type { ComponentType } from "svelte";
import RecipeListScreen from "../components/screens/RecipeListScreen.svelte";
import RecipeScreen from "../components/screens/RecipeScreen.svelte";

export const recipeListRoute = () => "/";
export const recipeRoute = (id: number | string) => `/recipe/${id}`;

/**
 * This object contains all the app's routes for constructing the router.
 */
export const routes: { [route: string]: ComponentType } = {
  [recipeListRoute()]: RecipeListScreen,
  [recipeRoute(":id")]: RecipeScreen,
};
