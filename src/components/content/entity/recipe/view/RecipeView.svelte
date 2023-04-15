<!--
@component
This component displays the content of a recipe.
-->

<script>
  import { recipeRepository } from "../../../../../services/store/repository/recipe-repository.ts";
  import { isLoading } from "../../../../../services/util/loadable.ts";
  import RecipeStepList from "../../recipe-step/list/RecipeStepList.svelte";

  /**
   * the id of the recipe
   * @type {number}
   */
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
