<script>
  import { recipeFileRepository } from "../../../../../services/repository/recipe-file-repository.ts";
  import { messages } from "../../../../../services/translation/en.ts";
  import { isLoading } from "../../../../../services/util/is-loading.ts";
  import SvelteButton from "../../../../element/SvelteButton.svelte";
  import SvelteForm from "../../../../element/form/SvelteForm.svelte";
  import RecipeFileEdit from "../edit/RecipeFileEdit.svelte";
  import RecipeFileView from "../view/RecipeFileView.svelte";

  export let recipeStepId;

  /**
   * @type {Readable<Loadable<number[]>>}
   */
  let list;

  $: list = recipeFileRepository.createListFilteredStore({
    condition: { recipeStepId },
    orderBy: [
      {
        column: "order",
      },
    ],
  });
</script>

{#if !isLoading($list)}
  <ol>
    {#each $list as id}
      <li>
        <RecipeFileView id="{id}" /><SvelteButton
          on:click="{() => recipeFileRepository.delete(id)}"
          >{messages.labels.actions.delete.format()}</SvelteButton
        >
      </li>
    {/each}
  </ol>
  <SvelteForm
    on:submit="{async ({ detail: { values } }) => {
      await recipeFileRepository.create({
        name: values.name,
        order: $list.length + 1,
        path: values.path,
        recipeStepId,
      });
    }}"
  >
    <RecipeFileEdit />
    <SvelteButton type="submit"
      >{messages.labels.actions.create.format()}</SvelteButton
    >
  </SvelteForm>
{/if}
