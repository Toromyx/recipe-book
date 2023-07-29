<!--
@component
This component displays the content of a recipe.
-->

<script>
  import { recipeRepository } from "../../../../../services/store/repository/recipe-repository.ts";
  import { messages } from "../../../../../services/translation/en.ts";
  import { isLoaded } from "../../../../../services/util/loadable.ts";
  import SvelteInput from "../../../../element/form/SvelteInput.svelte";
  import RecipeStepList from "../../recipe-step/list/RecipeStepList.svelte";

  /**
   * the id of the recipe
   * @type {number}
   */
  export let id;

  /** @type {Readable<Loadable<RecipeInterface>>} */
  let recipe;

  let factor = 1;

  $: recipe = recipeRepository.createStore(id);
</script>

{#if isLoaded($recipe)}
  <div>
    <h1>{$recipe.name}</h1>
    <SvelteInput
      on:input="{({ detail }) => (factor = detail)}"
      name="factor"
      type="number"
      label="{messages.labels.factor.resolveMessage().toString()}"
      value="{factor}"
      min="{0}"
    />
    <RecipeStepList recipeId="{id}" factor="{factor}" />
  </div>
{/if}
