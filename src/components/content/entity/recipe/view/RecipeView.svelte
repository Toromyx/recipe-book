<!--
@component
This component displays the content of a recipe.
-->

<script>
  import { recipeIngredientDraftRepository } from "../../../../../services/store/repository/recipe-ingredient-draft-repository.ts";
  import { recipeRepository } from "../../../../../services/store/repository/recipe-repository.ts";
  import { messages } from "../../../../../services/translation/en.ts";
  import { isLoaded } from "../../../../../services/util/loadable.ts";
  import SvelteInput from "../../../../element/form/SvelteInput.svelte";
  import RecipeFileList from "../../recipe-file/list/RecipeFileList.svelte";
  import RecipeStepList from "../../recipe-step/list/RecipeStepList.svelte";
  import DraftIngredientForm from "./RecipeView/DraftIngredientForm.svelte";

  /**
   * the id of the recipe
   * @type {number}
   */
  export let id;

  /** @type {Readable<Loadable<RecipeInterface>>} */
  let recipe;
  /** @type {Readable<Loadable<number[]>>} */
  let ingredientDraftList;

  let factor = 1;

  $: recipe = recipeRepository.createStore(id);
  $: ingredientDraftList =
    recipeIngredientDraftRepository.createListFilteredStore({
      condition: { recipeId: id },
      orderBy: [{ order: "asc" }],
    });
</script>

{#if isLoaded($recipe)}
  <div>
    <h1>{$recipe.name}</h1>
    <RecipeFileList recipeId="{id}" />
    <SvelteInput
      on:input="{({ detail }) => (factor = detail)}"
      name="factor"
      type="number"
      label="{messages.labels.factor.format()}"
      value="{factor}"
      min="{0}"
    />
    {#if isLoaded($ingredientDraftList)}
      {#each $ingredientDraftList as ingredientDraftId}
        <DraftIngredientForm
          recipeId="{id}"
          recipeIngredientDraftId="{ingredientDraftId}"
        />
      {/each}
    {/if}
    <RecipeStepList recipeId="{id}" factor="{factor}" />
  </div>
{/if}
