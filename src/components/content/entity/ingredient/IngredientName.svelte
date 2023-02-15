<script>
  import { onDestroy } from "svelte";
  import { ingredientRepository } from "../../../../services/repository/ingredient-repository.ts";

  export let id;

  /** @type {IngredientInterface | undefined} */
  let ingredient;
  let unsubscribe = () => {};

  $: {
    unsubscribe();
    if (id) {
      unsubscribe = ingredientRepository.subscribe(id, (entity) => {
        ingredient = entity;
      });
    }
  }

  onDestroy(() => {
    unsubscribe();
  });
</script>

<span>{ingredient?.name}</span>
