<!--
@component
This component lists all recipes with a form to create a new recipe.
-->

<script>
  import { link, push } from "svelte-spa-router";
  import { invoke } from "../../../../../services/command/client.ts";
  import { Command } from "../../../../../services/command/command.ts";
  import { getExternalRecipe } from "../../../../../services/external-recipe/getter.ts";
  import { recipeRoute } from "../../../../../services/router.ts";
  import { recipeIngredientDraftRepository } from "../../../../../services/store/repository/recipe-ingredient-draft-repository.ts";
  import { recipeRepository } from "../../../../../services/store/repository/recipe-repository.ts";
  import { recipeStepRepository } from "../../../../../services/store/repository/recipe-step-repository.ts";
  import { messages } from "../../../../../services/translation/en.ts";
  import { isLoading } from "../../../../../services/util/loadable.ts";
  import SvelteButton from "../../../../element/SvelteButton.svelte";
  import SvelteForm from "../../../../element/form/SvelteForm.svelte";
  import SvelteInput from "../../../../element/form/SvelteInput.svelte";
  import RecipeViewName from "../view/RecipeViewName.svelte";

  let input;
  let list = recipeRepository.createListStore();

  async function onSubmit({ detail: { values } }) {
    let url;
    try {
      url = new URL(values.name);
    } catch (e) {
      // not a valid url
      const recipeId = await recipeRepository.create({ name: values.name });
      await push(recipeRoute(recipeId));
      return;
    }
    let externalRecipeData;
    try {
      externalRecipeData = await invoke(Command.EXTERNAL_RECIPE, {
        url,
      });
    } catch (reason) {
      if (reason.ExternalRecipeUrlNotSupported) {
        input.setAndReportCustomValidity(
          messages.validity.externalRecipeUrlNotSupported.format(),
        );
        return;
      }
      throw reason;
    }
    const externalRecipe = await getExternalRecipe(externalRecipeData);
    const recipeId = await recipeRepository.create({
      name: externalRecipe.name,
    });
    await Promise.all(
      externalRecipe.steps.map(async (step, i) => {
        const recipeStepId = await recipeStepRepository.create({
          recipeId,
          description: step.description,
          order: i + 1,
        });
        await Promise.all(
          step.ingredients.map(async (ingredient, i) => {
            await recipeIngredientDraftRepository.create({
              recipeStepId,
              order: i + 1,
              text: ingredient,
            });
          }),
        );
      }),
    );
    await push(recipeRoute(recipeId));
  }
</script>

{#if !isLoading($list)}
  <ul>
    {#each $list as id}
      <li>
        <a href="{recipeRoute(id)}" use:link><RecipeViewName id="{id}" /></a
        ><SvelteButton
          on:click="{() => recipeRepository.delete(id)}"
          confirmation="{true}"
          >{messages.labels.actions.delete.format()}</SvelteButton
        >
      </li>
    {/each}
  </ul>
  <SvelteForm on:submit="{onSubmit}">
    <SvelteInput
      bind:this="{input}"
      name="name"
      required="{true}"
      label="{messages.labels.entityFields.recipe.name.format()}"
    />
    <SvelteButton type="submit"
      >{messages.labels.actions.create.format()}</SvelteButton
    >
  </SvelteForm>
{/if}
