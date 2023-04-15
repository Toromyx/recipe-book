<!--
@component
This component displays the content of a recipe step.
-->

<script>
  import { recipeStepRepository } from "../../../../../services/store/repository/recipe-step-repository.ts";
  import { messages } from "../../../../../services/translation/en.ts";
  import { isLoading } from "../../../../../services/util/loadable.ts";
  import Editable from "../../../../layout/Editable.svelte";
  import RecipeFileList from "../../recipe-file/list/RecipeFileList.svelte";
  import RecipeIngredientList from "../../recipe-ingredient/list/RecipeIngredientList.svelte";
  import RecipeStepEdit from "../edit/RecipeStepEdit.svelte";

  /**
   * the recipe step id
   * @type {number}
   */
  export let id;

  /** @type {Readable<Loadable<RecipeStepInterface>>} */
  let recipeStep;

  $: recipeStep = recipeStepRepository.createStore(id);
</script>

{#if !isLoading($recipeStep)}
  <h2>
    {messages.headings.recipeStep.format({ number: $recipeStep.order })}
  </h2>
  <h3>{messages.headings.ingredients.format()}</h3>
  <RecipeIngredientList recipeStepId="{id}" />
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
      <p>{$recipeStep.description}</p>
    </svelte:fragment>
    <RecipeStepEdit slot="edit" description="{$recipeStep.description}" />
  </Editable>
  <h3>{messages.headings.files.format()}</h3>
  <RecipeFileList recipeStepId="{id}" />
{/if}
