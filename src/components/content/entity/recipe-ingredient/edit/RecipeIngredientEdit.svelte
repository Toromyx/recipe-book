<!--
@component
This component implements form fields for editing an recipe ingredient.

# Events

The event `edit` is fired when the user makes yn change to any field. The event detail is all the form values.
-->

<script>
  import { createEventDispatcher } from "svelte";
  import {
    createIngredient,
    listIngredient,
  } from "../../../../../services/command/entity.ts";
  import { messages } from "../../../../../services/translation/en.ts";
  import Autocomplete from "../../../../element/form/Autocomplete.svelte";
  import SvelteInput from "../../../../element/form/SvelteInput.svelte";
  import { UNIT_LIST_ID } from "../../../UnitList.svelte";
  import IngredientViewName from "../../ingredient/view/IngredientViewName.svelte";

  /**
   * the existing recipe ingredient's quantity
   * @type {?number}
   */
  export let quantity = undefined;
  /**
   * the existing recipe ingredient's unit
   * @type {?string}
   */
  export let unit = undefined;
  /**
   * the existing recipe ingredient's ingredient id
   * @type {?number}
   */
  export let ingredientId = undefined;
  /**
   * the existing recipe ingredient's ingredient name
   * @type {string}
   */
  export let ingredientName = "";
  /**
   * the existing recipe ingredient's quality
   * @type {?string}
   */
  export let quality = undefined;
  /**
   * already used ingredient ids
   * @type {number[]}
   */
  export let usedIngredientIds = [];

  const dispatch = createEventDispatcher();

  let innerQuantity;
  let innerUnit;
  let innerIngredientName;
  let innerIngredientId;
  let innerQuality;

  $: innerQuantity = quantity;
  $: innerUnit = unit;
  $: innerIngredientName = ingredientName;
  $: innerIngredientId = ingredientId;
  $: innerQuality = quality;

  function onQuantity({ detail }) {
    innerQuantity = detail;
    onUserInput();
  }
  function onUnit({ detail }) {
    innerUnit = detail;
    onUserInput();
  }
  function onIngredientName({ detail }) {
    innerIngredientName = detail;
    onUserInput();
  }
  function onIngredientId({ detail }) {
    innerIngredientId = detail[0];
    onUserInput();
  }
  function onQuality({ detail }) {
    innerQuality = detail;
    onUserInput();
  }

  function onUserInput() {
    dispatch("edit", {
      quantity: innerQuantity,
      unit: innerUnit,
      ingredientName: innerIngredientName,
      ingredientId: innerIngredientId,
      quality: innerQuality,
    });
  }
</script>

<SvelteInput
  on:input="{onQuantity}"
  on:change="{onQuantity}"
  on:paste
  name="quantity"
  type="number"
  value="{innerQuantity}"
  label="{messages.labels.entityFields.recipeIngredient.quantity.format()}"
  min="0"
/>
<SvelteInput
  on:input="{onUnit}"
  on:change="{onUnit}"
  on:paste
  name="unit"
  value="{innerUnit}"
  label="{messages.labels.entityFields.recipeIngredient.unit.format()}"
  list="{UNIT_LIST_ID}"
/>
<Autocomplete
  on:input="{onIngredientName}"
  on:change="{onIngredientName}"
  on:select="{onIngredientId}"
  on:paste
  name="ingredientId"
  min="{1}"
  max="{1}"
  value="{innerIngredientId ? [innerIngredientId] : []}"
  userInput="{innerIngredientName}"
  excludedValues="{usedIngredientIds}"
  label="{messages.labels.entityFields.recipeIngredient.ingredient.format()}"
  callback="{(userInput) =>
    listIngredient({
      condition: { name: userInput },
      orderBy: [{ column: 'name' }],
    })}"
  createCallback="{(userInput) => createIngredient({ name: userInput })}"
  ><svelte:fragment let:item>
    <IngredientViewName id="{item}" />
  </svelte:fragment></Autocomplete
>
<SvelteInput
  on:input="{onQuality}"
  on:change="{onQuality}"
  on:paste
  name="quality"
  value="{innerQuality}"
  label="{messages.labels.entityFields.recipeIngredient.quality.format()}"
/>
