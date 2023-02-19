<script>
  import { onDestroy } from "svelte";
  import { recipeIngredientRepository } from "../../../../services/repository/recipe-ingredient-repository.ts";
  import { messages } from "../../../../services/translation/en.ts";
  import SvelteButton from "../../../element/SvelteButton.svelte";
  import SvelteForm from "../../../element/form/SvelteForm.svelte";
  import RecipeIngredient from "./RecipeIngredient.svelte";
  import RecipeIngredientFormFields from "./RecipeIngredientFormFields.svelte";

  export let recipeStepId;

  let list = [];
  let unsubscribe = () => {};
  $: {
    unsubscribe();
    unsubscribe = recipeIngredientRepository.subscribeListFiltered(
      {
        condition: { recipeStepId },
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
        <RecipeIngredient id="{id}" /><SvelteButton
          on:click="{() => recipeIngredientRepository.delete(id)}"
          >{messages.labels.actions.delete.format()}</SvelteButton
        >
      </li>
    {/each}
  </ol>
  <SvelteForm
    on:submit="{({ detail }) => {
      recipeIngredientRepository.create({
        order: list.length + 1,
        quantity: detail.quantity,
        unit: detail.unit,
        ingredientId: detail.ingredientId[0],
        recipeStepId,
      });
    }}"
  >
    <RecipeIngredientFormFields />
    <SvelteButton type="submit"
      >{messages.labels.actions.create.format()}</SvelteButton
    >
  </SvelteForm>
</div>
