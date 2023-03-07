<script>
  import { ingredientRepository } from "../../../../../services/repository/ingredient-repository.ts";
  import { isLoading } from "../../../../../services/util/is-loading.ts";

  export let id;

  /** @type {Readable<Loadable<IngredientInterface>>} */
  let ingredient;

  $: ingredient = ingredientRepository.createStore(id);
</script>

{#if !isLoading($ingredient)}
  <span>{$ingredient.name}</span>
{/if}
