<!--
@component
This component displays the heading of a recipe step.
-->

<script>
  import { recipeStepRepository } from "../../../../../services/store/repository/recipe-step-repository.ts";
  import { messages } from "../../../../../services/translation/en.ts";
  import { isLoaded } from "../../../../../services/util/loadable.ts";

  /**
   * the recipe step id
   * @type {number}
   */
  export let id;

  /** @type {Readable<Loadable<RecipeStepInterface>>} */
  let recipeStep;

  $: recipeStep = recipeStepRepository.createStore(id);
</script>

{#if isLoaded($recipeStep)}
  <span
    >{messages.headings.recipeStep.format({
      number: $recipeStep.order,
    })}</span
  >
{/if}
