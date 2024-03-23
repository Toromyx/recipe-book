<!--
@component

This component implements a form for assigning a recipe ingredient draft (connected to a recipe) to a recipe step, making a recipe step ingredient out of it.
-->

<script>
  import { parseString } from "../../../../../../services/parser/recipe-ingredient-parser.ts";
  import { ingredientRepository } from "../../../../../../services/store/repository/ingredient-repository.ts";
  import { recipeIngredientDraftRepository } from "../../../../../../services/store/repository/recipe-ingredient-draft-repository.ts";
  import { recipeStepIngredientRepository } from "../../../../../../services/store/repository/recipe-step-ingredient-repository.ts";
  import { recipeStepRepository } from "../../../../../../services/store/repository/recipe-step-repository.ts";
  import { unitList } from "../../../../../../services/store/unit-list.ts";
  import { messages } from "../../../../../../services/translation/en.ts";
  import {
    whenLoadingDefault,
    isLoaded,
    whenLoadedValue,
  } from "../../../../../../services/util/loadable.ts";
  import SvelteButton from "../../../../../element/SvelteButton.svelte";
  import SvelteForm from "../../../../../element/form/SvelteForm.svelte";
  import SvelteSelect from "../../../../../element/form/SvelteSelect.svelte";
  import RecipeStepViewHeading from "../../../recipe-step/view/RecipeStepViewHeading.svelte";
  import RecipeStepIngredientEdit from "../../../recipe-step-ingredient/edit/RecipeStepIngredientEdit.svelte";

  /** @type {number} */
  export let recipeIngredientDraftId;
  /** @type {number} */
  export let recipeId;

  /** @type {Readable<Loadable<RecipeIngredientDraftInterface>>} */
  let recipeIngredientDraft;
  /** @type {Loadable<ParsedRecipeIngredient & {id?: number}>} */
  let parsedRecipeIngredient;
  /** @type {Readable<Loadable<number[]>>} */
  let recipeStepList;
  /** @type {Readable<Loadable<number[]>>} */
  let ingredientList;
  /** @type {Readable<Loadable<number>>} */
  let recipeStepIngredientCount;
  /** @type {Loadable<number|undefined>} */
  let recipeStepId;

  $: recipeIngredientDraft = recipeIngredientDraftRepository.createStore(
    recipeIngredientDraftId,
  );
  $: parsedRecipeIngredient = whenLoadedValue(
    $recipeIngredientDraft,
    (recipeIngredientDraft) =>
      parseString(recipeIngredientDraft.text, $unitList),
  );
  $: recipeStepList = recipeStepRepository.createListFilteredStore({
    condition: { recipeId },
    orderBy: [{ order: "asc" }],
  });
  $: recipeStepId =
    recipeStepId === undefined
      ? whenLoadedValue($recipeStepList, (recipeStepList) => recipeStepList[0])
      : recipeStepId;
  $: ingredientList = ingredientRepository.createListFilteredStore({
    condition: { recipeStepId },
  });
  $: recipeStepIngredientCount =
    recipeStepIngredientRepository.createCountFilteredStore({
      recipeStepId,
    });
</script>

{#if isLoaded(parsedRecipeIngredient) && isLoaded($recipeStepList)}
  <SvelteForm
    on:submit="{async ({ detail: { values } }) => {
      if (!isLoaded($recipeStepIngredientCount)) {
        return;
      }
      await recipeStepIngredientRepository.create({
        order: $recipeStepIngredientCount + 1,
        quantity: values.quantity || null,
        unit: values.unit || null,
        quality: values.quality || null,
        ingredientId: values.ingredientId[0],
        recipeStepId: values.recipeStepId,
      });
      await recipeIngredientDraftRepository.delete(recipeIngredientDraftId);
    }}"
  >
    <RecipeStepIngredientEdit
      quantity="{parsedRecipeIngredient.quantity}"
      unit="{parsedRecipeIngredient.unit}"
      ingredientName="{parsedRecipeIngredient.name}"
      ingredientId="{parsedRecipeIngredient.id}"
      quality="{parsedRecipeIngredient.quality}"
      usedIngredientIds="{whenLoadingDefault($ingredientList, [])}"
    />
    <SvelteSelect
      on:change="{({ detail }) => {
        recipeStepId = detail;
      }}"
      value="{recipeStepId}"
      options="{$recipeStepList}"
      label="{messages.labels.entityNames.recipeStep.format()}"
      name="recipeStepId"
      required="{true}"
    >
      <svelte:fragment let:option>
        <RecipeStepViewHeading id="{option}" />
      </svelte:fragment>
    </SvelteSelect>
    <SvelteButton type="submit"
      >{messages.labels.actions.add.format()}</SvelteButton
    >
  </SvelteForm>
{/if}
