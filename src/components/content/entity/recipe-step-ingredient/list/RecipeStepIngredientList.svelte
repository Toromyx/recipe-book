<!--
@component
This component displays an ordered list of recipe step ingredients of a recipe step.

It provides functionality to add a new recipe step ingredient and integrates with the recipe ingredient parser.
It also displays displays the ingredients recipes drafts.
-->

<script>
  import { readRecipeStepIngredientDraft } from "../../../../../services/command/entity.ts";
  import {
    parseHtml,
    parseText,
    parseString,
  } from "../../../../../services/parser/recipe-ingredient-parser.ts";
  import { ingredientRepository } from "../../../../../services/store/repository/ingredient-repository.ts";
  import { recipeStepIngredientDraftRepository } from "../../../../../services/store/repository/recipe-step-ingredient-draft-repository.ts";
  import { recipeStepIngredientRepository } from "../../../../../services/store/repository/recipe-step-ingredient-repository.ts";
  import { unitList } from "../../../../../services/store/unit-list.ts";
  import { messages } from "../../../../../services/translation/en.ts";
  import {
    whenLoadingDefault,
    isLoaded,
  } from "../../../../../services/util/loadable.ts";
  import { updateOrder } from "../../../../../services/util/update-order.ts";
  import SvelteButton from "../../../../element/SvelteButton.svelte";
  import SvelteForm from "../../../../element/form/SvelteForm.svelte";
  import RecipeStepIngredientEdit from "../edit/RecipeStepIngredientEdit.svelte";
  import RecipeStepIngredientView from "../view/RecipeStepIngredientView.svelte";
  import AdditionalIngredientsForm from "./RecipeStepIngredientList/AdditionalIngredientsForm.svelte";

  /**
   * the recipe step id
   * @type {number}
   */
  export let recipeStepId;
  /** @type {number} */
  export let factor = 1;

  /** @type {Readable<Loadable<number[]>>} */
  let list;
  /** @type {Readable<Loadable<number[]>>} */
  let usedIngredientsList;
  /** @type {Readable<Loadable<number[]>>} */
  let draftList;
  /** @type {(ParsedRecipeIngredient & {id?: number})[]} */
  let pastedParsedRecipeIngredients = [];
  /** @type {Promise<Array<ParsedRecipeIngredient|null>>} */
  let draftedParsedRecipeIngredientsPromise = new Promise(() => {});

  $: list = recipeStepIngredientRepository.createListFilteredStore({
    condition: { recipeStepId },
    orderBy: [
      {
        order: "asc",
      },
    ],
  });
  $: usedIngredientsList = ingredientRepository.createListFilteredStore({
    condition: { recipeStepId },
  });
  $: draftList = recipeStepIngredientDraftRepository.createListFilteredStore({
    condition: { recipeStepId },
    orderBy: [{ order: "asc" }],
  });
  $: draftedParsedRecipeIngredientsPromise = isLoaded($draftList)
    ? Promise.all(
        $draftList.map(async (recipeStepIngredientDraftId) => {
          const recipeStepIngredientDraft = await readRecipeStepIngredientDraft(
            recipeStepIngredientDraftId,
          );
          return parseString(recipeStepIngredientDraft.text, $unitList);
        }),
      )
    : new Promise(() => {});
</script>

{#if isLoaded($list)}
  {#await draftedParsedRecipeIngredientsPromise then draftedParsedRecipeIngredients}
    {#if draftedParsedRecipeIngredients.length}
      <AdditionalIngredientsForm
        on:done="{async () => {
          for (const recipeStepIngredientDraftId of $draftList) {
            void recipeStepIngredientDraftRepository.delete(
              recipeStepIngredientDraftId,
            );
          }
          draftedParsedRecipeIngredients = [];
        }}"
        numIngredients="{$list.length}"
        recipeStepId="{recipeStepId}"
        usedIngredientIds="{$usedIngredientsList}"
        parsedRecipeIngredients="{draftedParsedRecipeIngredients.filter(
          Boolean,
        )}"
      />
    {/if}
  {/await}
  <ol>
    {#each $list as id (id)}
      <li>
        <RecipeStepIngredientView id="{id}" factor="{factor}" /><SvelteButton
          on:click="{async () => {
            await recipeStepIngredientRepository.delete(id);
            updateOrder(recipeStepIngredientRepository, $list, id);
          }}"
          confirmation="{true}"
          >{messages.labels.actions.delete.format()}</SvelteButton
        >
      </li>
    {/each}
  </ol>
  {#if pastedParsedRecipeIngredients.length}
    <AdditionalIngredientsForm
      on:done="{() => (pastedParsedRecipeIngredients = [])}"
      numIngredients="{$list.length}"
      recipeStepId="{recipeStepId}"
      usedIngredientIds="{$usedIngredientsList}"
      parsedRecipeIngredients="{pastedParsedRecipeIngredients}"
    />
  {/if}
  <SvelteForm
    on:submit="{async ({ detail: { values, context } }) => {
      await recipeStepIngredientRepository.create({
        order: $list.length + 1,
        quantity: values.quantity || null,
        unit: values.unit || null,
        quality: values.quality || null,
        ingredientId: values.ingredientId[0],
        recipeStepId,
      });
      context.reset();
    }}"
  >
    <RecipeStepIngredientEdit
      on:paste="{(e) => {
        e.preventDefault();
        const html = e.clipboardData.getData('text/html');
        if (html) {
          pastedParsedRecipeIngredients = parseHtml(
            html,
            whenLoadingDefault($unitList, []),
          );
          return;
        }
        const text = e.clipboardData.getData('text/plain');
        pastedParsedRecipeIngredients = parseText(
          text,
          whenLoadingDefault($unitList, []),
        );
      }}"
      usedIngredientIds="{whenLoadingDefault($usedIngredientsList, [])}"
    />
    <SvelteButton type="submit"
      >{messages.labels.actions.create.format()}</SvelteButton
    >
  </SvelteForm>
{/if}
