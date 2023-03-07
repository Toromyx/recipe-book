<script>
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
</script>

<SvelteInput
  on:paste
  name="quantity"
  type="number"
  value="{quantity}"
  label="{messages.labels.entityFields.recipeIngredient.quantity.format()}"
  min="0"
/>
<SvelteInput
  on:paste
  name="unit"
  value="{unit}"
  label="{messages.labels.entityFields.recipeIngredient.unit.format()}"
/>
<Autocomplete
  on:paste
  name="ingredientId"
  min="{1}"
  max="{1}"
  value="{ingredientId ? [ingredientId] : []}"
  initialInput="{ingredientName}"
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
