<!--
@component
This component displays the content of a recipe step.
-->

<script>
  import { recipeStepRepository } from "../../../../../services/store/repository/recipe-step-repository.ts";
  import { messages } from "../../../../../services/translation/en.ts";
  import { isLoaded } from "../../../../../services/util/loadable.ts";
  import Editable from "../../../../layout/Editable.svelte";
  import RecipeFileList from "../../recipe-file/list/RecipeFileList.svelte";
  import RecipeIngredientList from "../../recipe-ingredient/list/RecipeIngredientList.svelte";
  import RecipeStepEdit from "../edit/RecipeStepEdit.svelte";

  /**
   * the recipe step id
   * @type {number}
   */
  export let id;
  /** @type {number} */
  export let factor = 1;

  /** @type {Readable<Loadable<RecipeStepInterface>>} */
  let recipeStep;

  $: recipeStep = recipeStepRepository.createStore(id);
</script>

{#if isLoaded($recipeStep)}
  <h2>
    {messages.headings.recipeStep
      .resolveMessage({ number: $recipeStep.order })
      .toString()}
  </h2>
  <h3>{messages.headings.ingredients.resolveMessage().toString()}</h3>
  <RecipeIngredientList recipeStepId="{id}" factor="{factor}" />
  <h3>{messages.headings.description.resolveMessage().toString()}</h3>
  <Editable
    on:edit="{async ({ detail: { values, changed } }) => {
      const update = { id };
      if (changed.description) {
        update.description = values.description;
      }
      await recipeStepRepository.update(id, () => update);
    }}"
  >
    <svelte:fragment slot="display">
      <p style="white-space: pre">{$recipeStep.description}</p>
    </svelte:fragment>
    <RecipeStepEdit slot="edit" description="{$recipeStep.description}" />
  </Editable>
  <h3>{messages.headings.files.resolveMessage().toString()}</h3>
  <RecipeFileList recipeStepId="{id}" />
{/if}
