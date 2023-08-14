<!--
@component
This component lists all recipes with a form to create a new recipe.
-->

<script>
  import { link, push } from "svelte-spa-router";
  import {
    ExternalRecipeUrlNotSupportedError,
    getExternalRecipe,
  } from "../../../../../services/external-recipe.ts";
  import { recipeRoute } from "../../../../../services/router.ts";
  import { recipeRepository } from "../../../../../services/store/repository/recipe-repository.ts";
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
    let recipeId;
    loadingExternalRecipe = true;
    try {
      recipeId = await getExternalRecipe(String(url));
    } catch (reason) {
      if (reason instanceof ExternalRecipeUrlNotSupportedError) {
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
