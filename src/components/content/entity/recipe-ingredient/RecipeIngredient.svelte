<script>
  import { onDestroy } from "svelte";
  import { recipeIngredientRepository } from "../../../../services/repository/recipe-ingredient-repository.ts";
  import IngredientName from "../ingredient/IngredientName.svelte";

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

<span
  >{recipeIngredient?.quantity}&nbsp;{recipeIngredient?.unit}&nbsp;<IngredientName
    id="{recipeIngredient?.ingredientId}"
  /></span
>
