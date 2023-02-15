<script>
  import { onDestroy } from "svelte";
  import { recipeRepository } from "../../../../services/repository/recipe-repository.ts";
  import { recipeRoute } from "../../../../services/router.ts";
  import { messages } from "../../../../services/translation/en.ts";
  import SvelteButton from "../../../element/SvelteButton.svelte";
  import SvelteForm from "../../../element/form/SvelteForm.svelte";
  import SvelteInput from "../../../element/form/SvelteInput.svelte";
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
    <li>
      <a href="#{recipeRoute(id)}"><RecipeName id="{id}" /></a><SvelteButton
        on:click="{() => recipeRepository.delete(id)}"
        >{messages.labels.actions.delete.format()}</SvelteButton
      >
    </li>
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
