<!--
@component
This component displays the content of a recipe step.
-->

<script>
  import { recipeStepRepository } from "../../../../../services/store/repository/recipe-step-repository.ts";
  import { messages } from "../../../../../services/translation/en.ts";
  import { isLoaded } from "../../../../../services/util/loadable.ts";
  import Editable from "../../../../layout/Editable.svelte";
  import RecipeStepFileList from "../../recipe-step-file/list/RecipeStepFileList.svelte";
  import RecipeStepIngredientList from "../../recipe-step-ingredient/list/RecipeStepIngredientList.svelte";
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
    {messages.headings.recipeStep.format({ number: $recipeStep.order })}
  </h2>
  <h3>{messages.headings.ingredients.format()}</h3>
  <RecipeStepIngredientList recipeStepId="{id}" factor="{factor}" />
  <h3>{messages.headings.description.format()}</h3>
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
      <p style="white-space: pre-wrap">{$recipeStep.description}</p>
    </svelte:fragment>
    <RecipeStepEdit slot="edit" description="{$recipeStep.description}" />
  </Editable>
  <h3>{messages.headings.files.format()}</h3>
  <RecipeStepFileList recipeStepId="{id}" />
{/if}
