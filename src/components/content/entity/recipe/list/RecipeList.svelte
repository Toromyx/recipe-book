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
  import { recipeFileRepository } from "../../../../../services/store/repository/recipe-file-repository.ts";
  import { recipeIngredientDraftRepository } from "../../../../../services/store/repository/recipe-ingredient-draft-repository.ts";
  import { recipeRepository } from "../../../../../services/store/repository/recipe-repository.ts";
  import { recipeStepRepository } from "../../../../../services/store/repository/recipe-step-repository.ts";
  import { messages } from "../../../../../services/translation/en.ts";
  import { isLoaded } from "../../../../../services/util/loadable.ts";
  import ActionableList from "../../../../element/ActionableList.svelte";
  import SvelteButton from "../../../../element/SvelteButton.svelte";
  import SvelteProgress from "../../../../element/SvelteProgress.svelte";
  import SvelteForm from "../../../../element/form/SvelteForm.svelte";
  import SvelteInput from "../../../../element/form/SvelteInput.svelte";
  import RecipeViewName from "../view/RecipeViewName.svelte";

  let input;
  let list = recipeRepository.createListStore();
  /** @type {boolean} */
  let loadingExternalRecipe = false;

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
    loadingExternalRecipe = true;
    try {
      externalRecipeData = await invoke(Command.EXTERNAL_RECIPE, {
        url,
      });
    } catch (reason) {
      if (reason.ExternalRecipeUrlNotSupported) {
        input.setAndReportCustomValidity(
          messages.validity.externalRecipeUrlNotSupported
            .resolveMessage()
            .toString(),
        );
        return;
      }
      throw reason;
    } finally {
      loadingExternalRecipe = false;
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
        await Promise.all([
          ...step.ingredients.map(async (ingredient, i) => {
            await recipeIngredientDraftRepository.create({
              recipeStepId,
              order: i + 1,
              text: ingredient,
            });
          }),
          ...step.files.map(async (file, i) => {
            await recipeFileRepository.create({
              recipeStepId,
              order: i + 1,
              name: file,
              uri: { url: file },
            });
          }),
        ]);
      }),
    );
    await push(recipeRoute(recipeId));
  }
</script>

{#if isLoaded($list)}
  <ActionableList
    list="{$list}"
    actions="{[
      {
        callback: (ids) => {
          for (const id of ids) {
            void recipeRepository.delete(id);
          }
        },
        label: messages.labels.actions.delete.resolveMessage().toString(),
        labelAll: messages.labels.actions.deleteSelectedItems
          .resolveMessage()
          .toString(),
        confirmation: true,
      },
    ]}"
    ><svelte:fragment let:item
      ><a href="{recipeRoute(item)}" use:link><RecipeViewName id="{item}" /></a
      ></svelte:fragment
    ></ActionableList
  >
  <SvelteForm on:submit="{onSubmit}">
    <SvelteInput
      bind:this="{input}"
      name="name"
      required="{true}"
      label="{messages.labels.entityFields.recipe.name
        .resolveMessage()
        .toString()}"
    />
    <SvelteButton type="submit" disabled="{loadingExternalRecipe}"
      ><span aria-live="polite"
        >{#if loadingExternalRecipe}<SvelteProgress
            label="{messages.labels.descriptions.progress.loadingExternalRecipe
              .resolveMessage()
              .toString()}"
          />{:else}{messages.labels.actions.create
            .resolveMessage()
            .toString()}{/if}</span
      ></SvelteButton
    >
  </SvelteForm>
{/if}
