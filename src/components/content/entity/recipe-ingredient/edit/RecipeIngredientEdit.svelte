<script>
  import { createEventDispatcher } from "svelte";
  import { apiClient } from "../../../../../services/command/entity.ts";
  import { messages } from "../../../../../services/translation/en.ts";
  import Autocomplete from "../../../../element/form/Autocomplete.svelte";
  import SvelteInput from "../../../../element/form/SvelteInput.svelte";
  import IngredientViewName from "../../ingredient/view/IngredientViewName.svelte";

  export let quantity = undefined;
  export let unit = undefined;
  export let ingredientId = undefined;
  export let ingredientName = "";
  export let usedIngredientIds = undefined;

  const values = {
    quantity,
    unit,
    ingredientName,
    ingredientId,
  };
  const dispatch = createEventDispatcher();

  function onQuantity({ detail }) {
    values.quantity = detail;
    onUserInput();
  }
  function onUnit({ detail }) {
    values.unit = detail;
    onUserInput();
  }
  function onIngredientName({ detail }) {
    values.ingredientName = detail;
    onUserInput();
  }
  function onIngredientId({ detail }) {
    values.ingredientId = detail[0];
    onUserInput();
  }

  function onUserInput() {
    dispatch("edit", values);
  }
</script>

<SvelteInput
  on:input="{onQuantity}"
  on:change="{onQuantity}"
  on:paste
  name="quantity"
  type="number"
  value="{quantity}"
  label="{messages.labels.entityFields.recipeIngredient.quantity.format()}"
  min="0"
/>
<SvelteInput
  on:input="{onUnit}"
  on:change="{onUnit}"
  on:paste
  name="unit"
  value="{unit}"
  label="{messages.labels.entityFields.recipeIngredient.unit.format()}"
  list="unit-list"
/>
<Autocomplete
  on:input="{onIngredientName}"
  on:change="{onIngredientName}"
  on:select="{onIngredientId}"
  on:paste
  name="ingredientId"
  min="{1}"
  max="{1}"
  value="{ingredientId ? [ingredientId] : []}"
  userInput="{ingredientName}"
  excludedValues="{usedIngredientIds}"
  label="{messages.labels.entityFields.recipeIngredient.ingredient.format()}"
  callback="{(userInput) =>
    apiClient.listIngredient({
      condition: { name: userInput },
      orderBy: [{ column: 'name' }],
    })}"
  createCallback="{(userInput) =>
    apiClient.createIngredient({ name: userInput })}"
  ><svelte:fragment let:item>
    <IngredientViewName id="{item}" />
  </svelte:fragment></Autocomplete
>
