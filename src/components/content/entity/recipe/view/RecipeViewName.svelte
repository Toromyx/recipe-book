<script>
  import { recipeRepository } from "../../../../../services/repository/recipe-repository.ts";
  import { isLoading } from "../../../../../services/util/is-loading.ts";

  export let id;

  /** @type {Readable<Loadable<RecipeInterface>>} */
  let recipe;

  $: recipe = recipeRepository.createStore(id);
</script>

{#if !isLoading($recipe)}
  <span>{$recipe.name}</span>
{/if}
