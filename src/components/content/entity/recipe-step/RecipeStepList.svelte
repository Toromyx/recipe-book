<script>
  import { onDestroy } from "svelte";
  import { recipeStepRepository } from "../../../../services/repository/recipe-step-repository.ts";
  import { messages } from "../../../../services/translation/en.ts";
  import { getDataUrl } from "../../../../services/util/file.ts";
  import SvelteButton from "../../../element/SvelteButton.svelte";
  import SvelteForm from "../../../element/form/SvelteForm.svelte";
  import RecipeStep from "./RecipeStep.svelte";
  import RecipeStepFormFields from "./RecipeStepFormFields.svelte";

  export let recipeId;

  let list = [];
  let unsubscribe = () => {};

  $: {
    unsubscribe();
    unsubscribe = recipeStepRepository.subscribeListFiltered(
      {
        condition: { recipeId },
        orderBy: [
          {
            column: "order",
          },
        ],
      },
      (l) => {
        list = l;
      },
    );
  }

  onDestroy(() => {
    unsubscribe();
  });
</script>

<div>
  <ol>
    {#each list as id}
      <li>
        <RecipeStep id="{id}" /><SvelteButton
          on:click="{() => recipeStepRepository.delete(id)}"
          >{messages.labels.actions.delete.format()}</SvelteButton
        >
      </li>
    {/each}
  </ol>
  <SvelteForm
    on:submit="{async ({ detail: { values } }) => {
      const image = values.image ? await getDataUrl(values.image) : null;
      await recipeStepRepository.create({
        order: list.length + 1,
        description: values.description,
        image,
        recipeId,
      });
    }}"
  >
    <h3>{messages.headings.recipeStep.format({ number: list.length + 1 })}</h3>
    <RecipeStepFormFields />
    <SvelteButton type="submit"
      >{messages.labels.actions.create.format()}</SvelteButton
    >
  </SvelteForm>
</div>
