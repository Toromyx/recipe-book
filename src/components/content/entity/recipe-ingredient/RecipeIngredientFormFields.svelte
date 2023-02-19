<script>
  import { apiClient } from "../../../../services/command/entity.ts";
  import { messages } from "../../../../services/translation/en.ts";
  import Autocomplete from "../../../element/form/Autocomplete.svelte";
  import SvelteInput from "../../../element/form/SvelteInput.svelte";
  import IngredientName from "../ingredient/IngredientName.svelte";

  export let quantity = undefined;
  export let unit = undefined;
  export let ingredientId = undefined;
</script>

<SvelteInput
  name="quantity"
  type="number"
  value="{quantity}"
  label="{messages.labels.entityFields.recipeIngredient.quantity.format()}"
  required="{true}"
  min="0"
/>
<SvelteInput
  name="unit"
  value="{unit}"
  label="{messages.labels.entityFields.recipeIngredient.unit.format()}"
  required="{true}"
/>
<Autocomplete
  name="ingredientId"
  min="{1}"
  max="{1}"
  value="{ingredientId ? [ingredientId] : []}"
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
