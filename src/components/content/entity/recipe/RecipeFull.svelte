<script>
  import { recipeRepository } from "../../../../services/repository/recipe-repository.ts";
  import { messages } from "../../../../services/translation/en.ts";
  import RecipeStepList from "../recipe-step/RecipeStepList.svelte";

  export let id;

  /** @type {Readable<RecipeInterface | undefined>} */
  let recipe;

  $: recipe = recipeRepository.createStore(id);
</script>

<div>
  <h1>{$recipe?.name}</h1>
  <h2>{messages.headings.ingredients.format()}</h2>
  <RecipeStepList recipeId="{id}" />
</div>
