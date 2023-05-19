<!--
@component
This component displays the name of an recipe.
-->

<script>
  import { recipeRepository } from "../../../../../services/store/repository/recipe-repository.ts";
  import { isLoaded } from "../../../../../services/util/loadable.ts";

  /**
   * the id of the recipe
   * @type {number}
   */
  export let id;

  /** @type {Readable<Loadable<RecipeInterface>>} */
  let recipe;

  $: recipe = recipeRepository.createStore(id);
</script>

{#if isLoaded($recipe)}
  <span>{$recipe.name}</span>
{/if}
