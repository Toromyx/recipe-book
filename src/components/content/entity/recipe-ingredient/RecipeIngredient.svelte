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

  $: {
    unsubscribe();
    unsubscribe = recipeIngredientRepository.subscribe(id, (entity) => {
      recipeIngredient = entity;
    });
  }

  onDestroy(() => {
    unsubscribe();
  });
</script>

<Editable
  on:edit="{({ detail }) => {
    recipeIngredientRepository.update(id, () => ({
      id,
      unit: detail.unit,
      quantity: detail.quantity,
      ingredientId: detail.ingredientId[0],
    }));
  }}"
>
  <span slot="display"
    >{recipeIngredient?.quantity}&nbsp;{recipeIngredient?.unit}&nbsp;<IngredientName
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
