<script>
  import { recipeRoute } from "../../../../../services/router.ts";
  import { recipeRepository } from "../../../../../services/store/repository/recipe-repository.ts";
  import { messages } from "../../../../../services/translation/en.ts";
  import { isLoading } from "../../../../../services/util/is-loading.ts";
  import SvelteButton from "../../../../element/SvelteButton.svelte";
  import SvelteForm from "../../../../element/form/SvelteForm.svelte";
  import SvelteInput from "../../../../element/form/SvelteInput.svelte";
  import RecipeViewName from "../view/RecipeViewName.svelte";

  let list = recipeRepository.createListStore();
</script>

{#if !isLoading($list)}
  <ul>
    {#each $list as id}
      <li>
        <a href="#{recipeRoute(id)}"><RecipeViewName id="{id}" /></a
        ><SvelteButton
          on:click="{() => recipeRepository.delete(id)}"
          confirmation="{true}"
          >{messages.labels.actions.delete.format()}</SvelteButton
        >
      </li>
    {/each}
  </ul>
  <SvelteForm
    on:submit="{async ({ detail: { values, context } }) => {
      await recipeRepository.create({ name: values.name });
      context.reset();
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
{/if}
