<script>
  import { recipeRepository } from "../../../../../services/repository/recipe-repository.ts";
  import { isLoading } from "../../../../../services/util/is-loading.ts";
  import RecipeStepList from "../../recipe-step/list/RecipeStepList.svelte";

  export let id;

  /** @type {Readable<Loadable<RecipeInterface>>} */
  let recipe;

  $: recipe = recipeRepository.createStore(id);
</script>

{#if !isLoading($recipe)}
  <div>
    <h1>{$recipe.name}</h1>
    <RecipeStepList recipeId="{id}" />
  </div>
{/if}
