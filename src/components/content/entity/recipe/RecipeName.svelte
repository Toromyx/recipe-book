<script>
  import { recipeRepository } from "../../../../services/repository/recipe-repository.ts";

  export let id;

  /** @type {Readable<RecipeInterface | undefined>} */
  let recipe;

  $: recipe = recipeRepository.createStore(id);
</script>

<span>{$recipe?.name}</span>
