<script>
  import { onDestroy } from "svelte";
  import { apiClient } from "../../../../services/command/entity.ts";
  import { recipeIngredientRepository } from "../../../../services/repository/recipe-ingredient-repository.ts";
  import { messages } from "../../../../services/translation/en.ts";
  import SvelteButton from "../../../element/SvelteButton.svelte";
  import Autocomplete from "../../../element/form/Autocomplete.svelte";
  import SvelteForm from "../../../element/form/SvelteForm.svelte";
  import SvelteInput from "../../../element/form/SvelteInput.svelte";
  import IngredientName from "../ingredient/IngredientName.svelte";
  import RecipeIngredient from "./RecipeIngredient.svelte";

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
    <SvelteInput
      name="quantity"
      type="number"
      label="{messages.labels.entityFields.recipeIngredient.quantity.format()}"
      required="{true}"
      min="0"
    />
    <SvelteInput
      name="unit"
      label="{messages.labels.entityFields.recipeIngredient.unit.format()}"
      required="{true}"
    />
    <Autocomplete
      name="ingredientId"
      min="{1}"
      max="{1}"
      label="{messages.labels.entityFields.recipeIngredient.ingredient.format()}"
      callback="{(userInput) =>
        apiClient.listIngredient({
          condition: { name: userInput },
          orderBy: [{ column: 'name' }],
        })}"
      createCallback="{(userInput) =>
        apiClient.createIngredient({ name: userInput })}"
      ><svelte:fragment let:item>
        <IngredientName id="{item}" />
      </svelte:fragment></Autocomplete
    >
    <SvelteButton type="submit"
      >{messages.labels.actions.create.format()}</SvelteButton
    >
  </SvelteForm>
</div>
