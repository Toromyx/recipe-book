<script>
  import { onDestroy } from "svelte";
  import {
    parseHtml,
    parseText,
  } from "../../../../services/parser/recipe-ingredient-parser.ts";
  import { recipeIngredientRepository } from "../../../../services/repository/recipe-ingredient-repository.ts";
  import { messages } from "../../../../services/translation/en.ts";
  import SvelteButton from "../../../element/SvelteButton.svelte";
  import FieldListItem from "../../../element/form/FieldListItem.svelte";
  import SvelteFieldset from "../../../element/form/SvelteFieldset.svelte";
  import SvelteForm from "../../../element/form/SvelteForm.svelte";
  import RecipeIngredient from "./RecipeIngredient.svelte";
  import RecipeIngredientFormFields from "./RecipeIngredientFormFields.svelte";

  export let recipeStepId;

  let list = [];
  let unsubscribe = () => {};
  /** @type {ParsedRecipeIngredient[]} */
  let pastedParsedRecipeIngredients = [];

  $: {
    unsubscribe();
    unsubscribe = recipeIngredientRepository.subscribeListFiltered(
      {
        condition: { recipeStepId },
        orderBy: [
          {
            column: "order",
          },
        ],
      },
      (l) => {
        list = l;
      },
    );
  }

  onDestroy(() => {
    unsubscribe();
  });
</script>

<div>
  <ol>
    {#each list as id}
      <li>
        <RecipeIngredient id="{id}" /><SvelteButton
          on:click="{() => recipeIngredientRepository.delete(id)}"
          >{messages.labels.actions.delete.format()}</SvelteButton
        >
      </li>
    {/each}
  </ol>
  {#if pastedParsedRecipeIngredients.length}
    <SvelteForm
      on:submit="{({ detail: { values } }) => {
        for (let i = 0; i < values.ingredients.length; i++) {
          const ingredient = values.ingredients[i];
          recipeIngredientRepository.create({
            order: list.length + 1 + i,
            quantity: ingredient.quantity,
            unit: ingredient.unit,
            ingredientId: ingredient.ingredientId[0],
            recipeStepId,
          });
        }
        pastedParsedRecipeIngredients = [];
      }}"
    >
      <SvelteFieldset name="ingredients" isList="{true}">
        <ol>
          {#each pastedParsedRecipeIngredients as parsedRecipeIngredient, i}
            <li>
              <FieldListItem id="{i}">
                <RecipeIngredientFormFields
                  quantity="{parsedRecipeIngredient.quantity}"
                  unit="{parsedRecipeIngredient.unit}"
                  ingredientName="{parsedRecipeIngredient.name}"
                />
              </FieldListItem>
            </li>
          {/each}
        </ol>
      </SvelteFieldset>
      <SvelteButton type="submit"
        >{messages.labels.actions.create.format()}</SvelteButton
      >
      <SvelteButton on:click="{() => (pastedParsedRecipeIngredients = [])}"
        >{messages.labels.actions.cancel.format()}</SvelteButton
      >
    </SvelteForm>
  {/if}
  <SvelteForm
    on:submit="{({ detail: { values } }) => {
      recipeIngredientRepository.create({
        order: list.length + 1,
        quantity: values.quantity,
        unit: values.unit,
        ingredientId: values.ingredientId[0],
        recipeStepId,
      });
    }}"
  >
    <RecipeIngredientFormFields
      on:paste="{(e) => {
        e.preventDefault();
        const html = e.clipboardData.getData('text/html');
        if (html) {
          pastedParsedRecipeIngredients = parseHtml(html);
          return;
        }
        const text = e.clipboardData.getData('text/text');
        pastedParsedRecipeIngredients = parseText(text);
      }}"
    />
    <SvelteButton type="submit"
      >{messages.labels.actions.create.format()}</SvelteButton
    >
  </SvelteForm>
</div>
