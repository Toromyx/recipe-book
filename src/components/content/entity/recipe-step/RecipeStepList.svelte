<script>
  import { onDestroy } from "svelte";
  import { recipeStepRepository } from "../../../../services/repository/recipe-step-repository.ts";
  import { messages } from "../../../../services/translation/en.ts";
  import SvelteForm from "../../../element/form/SvelteForm.svelte";
  import SvelteButton from "../../../element/SvelteButton.svelte";
  import SvelteTextarea from "../../../element/form/SvelteTextarea.svelte";
  import RecipeStep from "./RecipeStep.svelte";

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
        <RecipeStep id="{id}" />
      </li>
    {/each}
  </ol>
  <SvelteForm
    on:submit="{({ detail }) => {
      recipeStepRepository.create({
        order: list.length + 1,
        description: detail.description,
        recipeId,
      });
    }}"
  >
    <h3>{messages.headings.recipeStep.format({ number: list.length + 1 })}</h3>
    <SvelteTextarea
      name="description"
      label="{messages.labels.entityFields.recipeStep.description.format()}"
    />
    <SvelteButton type="submit"
      >{messages.labels.actions.create.format()}</SvelteButton
    >
  </SvelteForm>
</div>
