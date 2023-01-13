<script>
  import { recipeRepository } from "../../../../services/repository/recipe-repository.js";
  import { onDestroy } from "svelte";
  import { recipeRoute } from "../../../../services/router.js";
  import SvelteForm from "../../../element/form/SvelteForm.svelte";
  import SvelteInput from "../../../element/form/SvelteInput.svelte";
  import { messages } from "../../../../services/translation/en.js";
  import SvelteButton from "../../../element/SvelteButton.svelte";
  import RecipeName from "./RecipeName.svelte";

  let list = [];

  let unsubscribe = recipeRepository.subscribeList((l) => {
    list = l;
  });

  onDestroy(() => {
    unsubscribe();
  });
</script>

<ol>
  {#each list as id}
    <li><a href="#{recipeRoute(id)}"><RecipeName id="{id}" /></a></li>
  {/each}
  <SvelteForm
    on:submit="{({ detail }) => {
      recipeRepository.create({ name: detail.name });
    }}"
  >
    <SvelteInput
      name="name"
      required="{true}"
      label="{messages.labels.entityFields.recipe.name.format()}"
    />
    <SvelteButton type="submit"
      >{messages.labels.actions.create.format()}</SvelteButton
    >
  </SvelteForm>
</ol>
