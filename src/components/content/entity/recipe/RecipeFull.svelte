<script>
  import { onDestroy } from "svelte";
  import { recipeRepository } from "../../../../services/repository/recipe-repository.ts";
  import { messages } from "../../../../services/translation/en.ts";
  import RecipeStepList from "../recipe-step/RecipeStepList.svelte";

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

<div>
  <h1>{recipe?.name}</h1>
  <h2>{messages.headings.ingredients.format()}</h2>
  <RecipeStepList recipeId="{id}" />
</div>
