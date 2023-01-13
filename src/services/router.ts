import RecipeListScreen from "../components/screens/RecipeListScreen.svelte";
import RecipeScreen from "../components/screens/RecipeScreen.svelte";

export const recipeListRoute = () => "/";
export const recipeRoute = (id: number | string) => `/recipe/${id}`;
export const routes = {
  [recipeListRoute()]: RecipeListScreen,
  [recipeRoute(":id")]: RecipeScreen,
};
