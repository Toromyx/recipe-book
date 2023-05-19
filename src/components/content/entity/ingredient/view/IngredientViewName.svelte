<!--
@component
This component displays the name of an ingredient.
-->

<script>
  import { ingredientRepository } from "../../../../../services/store/repository/ingredient-repository.ts";
  import { isLoaded } from "../../../../../services/util/loadable.ts";

  /**
   * the id of the ingredient
   * @type {number}
   */
  export let id;

  /** @type {Readable<Loadable<IngredientInterface>>} */
  let ingredient;

  $: ingredient = ingredientRepository.createStore(id);
</script>

{#if isLoaded($ingredient)}
  <span>{$ingredient.name}</span>
{/if}
