<script>
  import { recipeStepRepository } from "../../../../services/repository/recipe-step-repository.ts";
  import { messages } from "../../../../services/translation/en.ts";
  import Editable from "../../../layout/Editable.svelte";
  import RecipeFileList from "../recipe-file/list/RecipeFileList.svelte";
  import RecipeIngredientList from "../recipe-ingredient/RecipeIngredientList.svelte";
  import RecipeStepFormFields from "./RecipeStepFormFields.svelte";

  export let id;

  /** @type {Readable<RecipeStepInterface | undefined>} */
  let recipeStep;

  $: recipeStep = recipeStepRepository.createStore(id);
</script>

<h3>
  {messages.headings.recipeStep.format({ number: $recipeStep?.order })}
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
    <p>{$recipeStep?.description}</p>
  </svelte:fragment>
  <RecipeStepFormFields slot="edit" description="{$recipeStep?.description}" />
</Editable>
<RecipeFileList recipeStepId="{id}" />
