<script>
  import { recipeStepRepository } from "../../../../../services/repository/recipe-step-repository.ts";
  import { messages } from "../../../../../services/translation/en.ts";
  import { isLoading } from "../../../../../services/util/is-loading.ts";
  import Editable from "../../../../layout/Editable.svelte";
  import RecipeFileList from "../../recipe-file/list/RecipeFileList.svelte";
  import RecipeIngredientList from "../../recipe-ingredient/list/RecipeIngredientList.svelte";
  import RecipeStepEdit from "../edit/RecipeStepEdit.svelte";

  export let id;

  /** @type {Readable<Loadable<RecipeStepInterface>>} */
  let recipeStep;

  $: recipeStep = recipeStepRepository.createStore(id);
</script>

{#if !isLoading($recipeStep)}
  <h3>
    {messages.headings.recipeStep.format({ number: $recipeStep.order })}
  </h3>
  <RecipeIngredientList recipeStepId="{id}" />
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
  <RecipeFileList recipeStepId="{id}" />
{/if}
