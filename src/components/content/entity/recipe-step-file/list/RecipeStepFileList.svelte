<!--
@component
This component display an ordered list of all recipe step files of a recipe step.
-->

<script>
  import { recipeStepFileRepository } from "../../../../../services/store/repository/recipe-step-file-repository.ts";
  import { messages } from "../../../../../services/translation/en.ts";
  import { isLoaded } from "../../../../../services/util/loadable.ts";
  import { updateOrder } from "../../../../../services/util/update-order.ts";
  import SvelteButton from "../../../../element/SvelteButton.svelte";
  import SvelteForm from "../../../../element/form/SvelteForm.svelte";
  import RecipeStepFileEdit from "../edit/RecipeStepFileEdit.svelte";
  import RecipeStepFileView from "../view/RecipeStepFileView.svelte";

  /**
   * the id of the recipe step
   * @type {number}
   */
  export let recipeStepId;

  /**
   * @type {Readable<Loadable<number[]>>}
   */
  let list;

  $: list = recipeStepFileRepository.createListFilteredStore({
    condition: { recipeStepId },
    orderBy: [
      {
        order: "asc",
      },
    ],
  });
</script>

{#if isLoaded($list)}
  <ol>
    {#each $list as id (id)}
      <li>
        <RecipeStepFileView id="{id}" />
        <SvelteButton
          on:click="{async () => {
            await recipeStepFileRepository.delete(id);
            updateOrder(recipeStepFileRepository, $list, id);
          }}"
          confirmation="{true}"
          >{messages.labels.actions.delete.format()}</SvelteButton
        >
      </li>
    {/each}
  </ol>
  <SvelteForm
    on:submit="{async ({ detail: { values, context } }) => {
      await recipeStepFileRepository.create({
        name: values.name,
        order: $list.length + 1,
        uri: { path: values.path },
        recipeStepId,
      });
      context.reset();
    }}"
  >
    <RecipeStepFileEdit />
    <SvelteButton type="submit"
      >{messages.labels.actions.create.format()}</SvelteButton
    >
  </SvelteForm>
{/if}
