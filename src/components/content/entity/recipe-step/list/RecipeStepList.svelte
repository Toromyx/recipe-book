<script>
  import { recipeStepRepository } from "../../../../../services/repository/recipe-step-repository.ts";
  import { messages } from "../../../../../services/translation/en.ts";
  import { isLoading } from "../../../../../services/util/is-loading.ts";
  import SvelteButton from "../../../../element/SvelteButton.svelte";
  import SvelteForm from "../../../../element/form/SvelteForm.svelte";
  import RecipeStepEdit from "../edit/RecipeStepEdit.svelte";
  import RecipeStepView from "../view/RecipeStepView.svelte";

  export let recipeId;

  /** @type {Readable<Loadable<number[]>>} */
  let list;

  $: list = recipeStepRepository.createListFilteredStore({
    condition: { recipeId },
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
        <RecipeStepView id="{id}" /><SvelteButton
          on:click="{() => recipeStepRepository.delete(id)}"
          >{messages.labels.actions.delete.format()}</SvelteButton
        >
      </li>
    {/each}
  </ol>
  <SvelteForm
    on:submit="{async ({ detail: { values } }) => {
      await recipeStepRepository.create({
        order: $list.length + 1,
        description: values.description,
        recipeId,
      });
    }}"
  >
    <h3>{messages.headings.recipeStep.format({ number: $list.length + 1 })}</h3>
    <RecipeStepEdit />
    <SvelteButton type="submit"
      >{messages.labels.actions.create.format()}</SvelteButton
    >
  </SvelteForm>
{/if}
