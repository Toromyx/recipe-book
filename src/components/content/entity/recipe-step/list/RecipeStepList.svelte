<script>
  import { recipeStepRepository } from "../../../../../services/store/repository/recipe-step-repository.ts";
  import { messages } from "../../../../../services/translation/en.ts";
  import { isLoading } from "../../../../../services/util/is-loading.ts";
  import { updateOrder } from "../../../../../services/util/update-order.ts";
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
  {#each $list as id}
    <RecipeStepView id="{id}" /><SvelteButton
      on:click="{async () => {
        await recipeStepRepository.delete(id);
        updateOrder(recipeStepRepository, $list, id);
      }}"
      confirmation="{true}"
      >{messages.labels.actions.delete.format()}</SvelteButton
    >
  {/each}
  <SvelteForm
    on:submit="{async ({ detail: { values, context } }) => {
      await recipeStepRepository.create({
        order: $list.length + 1,
        description: values.description,
        recipeId,
      });
      context.reset();
    }}"
  >
    <h2>{messages.headings.recipeStep.format({ number: $list.length + 1 })}</h2>
    <RecipeStepEdit />
    <SvelteButton type="submit"
      >{messages.labels.actions.create.format()}</SvelteButton
    >
  </SvelteForm>
{/if}
