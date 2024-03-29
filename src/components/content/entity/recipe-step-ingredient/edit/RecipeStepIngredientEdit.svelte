<!--
@component
This component implements form fields for editing an recipe step ingredient.

# Events

The event `edit` is fired when the user makes a change to any field. The event detail is all the form values.
-->

<script>
  import { createEventDispatcher } from "svelte";
  import { listIngredient } from "../../../../../services/command/entity.ts";
  import { ingredientRepository } from "../../../../../services/store/repository/ingredient-repository.ts";
  import { messages } from "../../../../../services/translation/en.ts";
  import { whenLoadingDefault } from "../../../../../services/util/loadable.ts";
  import Autocomplete from "../../../../element/form/Autocomplete.svelte";
  import SvelteInput from "../../../../element/form/SvelteInput.svelte";
  import { UNIT_LIST_ID } from "../../../UnitList.svelte";
  import IngredientViewName from "../../ingredient/view/IngredientViewName.svelte";

  /**
   * the existing recipe step ingredient's quantity
   * @type {?number}
   */
  export let quantity = undefined;
  /**
   * the existing recipe step ingredient's unit
   * @type {?string}
   */
  export let unit = undefined;
  /**
   * the existing recipe step ingredient's ingredient id
   * @type {?number}
   */
  export let ingredientId = undefined;
  /**
   * the existing recipe step ingredient's ingredient name
   * @type {string}
   */
  export let ingredientName = "";
  /**
   * the existing recipe step ingredient's quality
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

  let ingredientIdUserInput = ingredientName;
  /**
   * @type {Readable<Loadable<number[]>>}
   */
  let ingredientIdResults;

  $: innerQuantity = quantity;
  $: innerUnit = unit;
  $: innerIngredientName = ingredientName;
  $: innerIngredientId = ingredientId;
  $: innerQuality = quality;
  $: ingredientIdResults = ingredientRepository.createListFilteredStore({
    condition: { name: ingredientIdUserInput },
    orderBy: [{ name: "asc" }],
  });

  // if no ingredient id is provided, try to find an exact match and set it
  if (!ingredientId && ingredientName) {
    void listIngredient({
      condition: { nameExact: ingredientName },
    }).then((list) => {
      if (list.length === 1) {
        innerIngredientId = list[0];
      }
    });
  }

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
  label="{messages.labels.entityFields.recipeStepIngredient.quantity.format()}"
  min="0"
/>
<SvelteInput
  on:input="{onUnit}"
  on:change="{onUnit}"
  on:paste
  name="unit"
  value="{innerUnit}"
  label="{messages.labels.entityFields.recipeStepIngredient.unit.format()}"
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
  label="{messages.labels.entityFields.recipeStepIngredient.ingredient.format()}"
  callback="{async (userInput) => (ingredientIdUserInput = userInput)}"
  results="{whenLoadingDefault($ingredientIdResults, [])}"
  createCallback="{(userInput) =>
    ingredientRepository.create({ name: userInput })}"
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
  label="{messages.labels.entityFields.recipeStepIngredient.quality.format()}"
/>
