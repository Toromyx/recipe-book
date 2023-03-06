<script>
  import { ingredientRepository } from "../../../../services/repository/ingredient-repository.ts";

  export let id;

  /** @type {Readable<IngredientInterface | undefined>} */
  let ingredient;

  $: ingredient = ingredientRepository.createStore(id);
</script>

<span>{$ingredient?.name}</span>
