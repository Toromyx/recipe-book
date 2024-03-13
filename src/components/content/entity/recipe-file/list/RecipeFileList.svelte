<!--
@component
This component display an ordered list of all recipe files of a recipe.
-->

<script>
  import { fileRepository } from "../../../../../services/store/repository/file-repository.ts";
  import { recipeFileRepository } from "../../../../../services/store/repository/recipe-file-repository.ts";
  import { messages } from "../../../../../services/translation/en.ts";
  import { isLoaded } from "../../../../../services/util/loadable.ts";
  import { updateOrder } from "../../../../../services/util/update-order.ts";
  import SvelteButton from "../../../../element/SvelteButton.svelte";
  import SvelteForm from "../../../../element/form/SvelteForm.svelte";
  import FileEdit from "../../file/edit/FileEdit.svelte";
  import RecipeStepFileView from "../view/RecipeFileView.svelte";

  /**
   * the id of the recipe step
   * @type {number}
   */
  export let recipeId;

  /**
   * @type {Readable<Loadable<number[]>>}
   */
  let list;

  $: list = recipeFileRepository.createListFilteredStore({
    condition: { recipeId },
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
            await recipeFileRepository.delete(id);
            updateOrder(recipeFileRepository, $list, id);
          }}"
          confirmation="{true}"
          >{messages.labels.actions.delete.format()}</SvelteButton
        >
      </li>
    {/each}
  </ol>
  <SvelteForm
    on:submit="{async ({ detail: { values, context } }) => {
      const fileId = await fileRepository.create({
        name: values.name,
        uri: { path: values.path },
      });
      await recipeFileRepository.create({
        order: $list.length + 1,
        recipeId,
        fileId,
      });
      context.reset();
    }}"
  >
    <FileEdit />
    <SvelteButton type="submit"
      >{messages.labels.actions.create.format()}</SvelteButton
    >
  </SvelteForm>
{/if}
