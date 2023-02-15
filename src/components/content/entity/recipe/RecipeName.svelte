<script>
  import { onDestroy } from "svelte";
  import { recipeRepository } from "../../../../services/repository/recipe-repository.ts";

  export let id;

  /** @type {RecipeInterface | undefined} */
  let recipe;
  let unsubscribe = () => {};

  $: {
    unsubscribe();
    unsubscribe = recipeRepository.subscribe(id, (entity) => {
      recipe = entity;
    });
  }

  onDestroy(() => {
    unsubscribe();
  });
</script>

<span>{recipe?.name}</span>
