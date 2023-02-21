<script>
  import { onDestroy } from "svelte";
  import { recipeIngredientRepository } from "../../../../services/repository/recipe-ingredient-repository.ts";
  import Editable from "../../../layout/Editable.svelte";
  import IngredientName from "../ingredient/IngredientName.svelte";
  import RecipeIngredientForm from "./RecipeIngredientFormFields.svelte";

  export let id;

  /** @type {RecipeIngredientInterface | undefined} */
  let recipeIngredient;
  let unsubscribe = () => {};
  let qualifiers;

  $: {
    unsubscribe();
    unsubscribe = recipeIngredientRepository.subscribe(id, (entity) => {
      recipeIngredient = entity;
    });
  }
  $: qualifiers = [recipeIngredient?.quantity, recipeIngredient?.unit].filter(
    Boolean,
  );

  onDestroy(() => {
    unsubscribe();
  });
</script>

<Editable
  on:edit="{({ detail: { values, changed } }) => {
    const update = { id };
    if (changed.unit) {
      update.unit = values.unit || null;
    }
    if (changed.quantity) {
      update.quantity = values.quantity || null;
    }
    if (changed.ingredientId) {
      update.ingredientId = values.ingredientId[0];
    }
    recipeIngredientRepository.update(id, () => update);
  }}"
>
  <span slot="display"
    >{#each qualifiers as qualifier}{qualifier}&nbsp;{/each}<IngredientName
      id="{recipeIngredient?.ingredientId}"
    /></span
  >
  <RecipeIngredientForm
    slot="edit"
    quantity="{recipeIngredient?.quantity}"
    unit="{recipeIngredient?.unit}"
    ingredientId="{recipeIngredient?.ingredientId}"
  />
</Editable>
