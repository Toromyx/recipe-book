<script>
  import { onDestroy } from "svelte";
  import { recipeStepRepository } from "../../../../services/repository/recipe-step-repository.ts";
  import { messages } from "../../../../services/translation/en.ts";
  import RecipeIngredientList from "../recipe-ingredient/RecipeIngredientList.svelte";

  export let id;

  /** @type {RecipeStepInterface | undefined} */
  let recipeStep;
  let unsubscribe = () => {};

  $: {
    unsubscribe();
    unsubscribe = recipeStepRepository.subscribe(id, (entity) => {
      recipeStep = entity;
    });
  }

  onDestroy(() => {
    unsubscribe();
  });
</script>

<div>
  <h3>
    {messages.headings.recipeStep.format({ number: recipeStep?.order })}
  </h3>
  <RecipeIngredientList recipeStepId="{id}" />
  <p>{recipeStep?.description}</p>
</div>
