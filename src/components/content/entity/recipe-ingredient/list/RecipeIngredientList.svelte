<script>
  import {
    parseHtml,
    parseText,
  } from "../../../../../services/parser/recipe-ingredient-parser.ts";
  import { ingredientRepository } from "../../../../../services/store/repository/ingredient-repository.ts";
  import { recipeIngredientRepository } from "../../../../../services/store/repository/recipe-ingredient-repository.ts";
  import { messages } from "../../../../../services/translation/en.ts";
  import { isLoading } from "../../../../../services/util/is-loading.ts";
  import { updateOrder } from "../../../../../services/util/update-order.ts";
  import SvelteButton from "../../../../element/SvelteButton.svelte";
  import FieldListItem from "../../../../element/form/FieldListItem.svelte";
  import SvelteFieldset from "../../../../element/form/SvelteFieldset.svelte";
  import SvelteForm from "../../../../element/form/SvelteForm.svelte";
  import RecipeIngredientEdit from "../edit/RecipeIngredientEdit.svelte";
  import RecipeIngredientView from "../view/RecipeIngredientView.svelte";

  export let recipeStepId;

  /** @type {Readable<Loadable<number[]>>} */
  let list;
  /** @type {Readable<Loadable<number[]>>} */
  let usedIngredientsList;
  /** @type {ParsedRecipeIngredient[]} */
  let pastedParsedRecipeIngredients = [];

  $: list = recipeIngredientRepository.createListFilteredStore({
    condition: { recipeStepId },
    orderBy: [
      {
        column: "order",
      },
    ],
  });
  $: usedIngredientsList = ingredientRepository.createListFilteredStore({
    condition: { recipeStepId },
  });
</script>

{#if !isLoading($list)}
  <ol>
    {#each $list as id}
      <li>
        <RecipeIngredientView id="{id}" /><SvelteButton
          on:click="{async () => {
            await recipeIngredientRepository.delete(id);
            updateOrder(recipeIngredientRepository, $list, id);
          }}"
          confirmation="{true}"
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
            order: $list.length + 1 + i,
            quantity: ingredient.quantity || null,
            unit: ingredient.unit || null,
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
                <RecipeIngredientEdit
                  quantity="{parsedRecipeIngredient.quantity}"
                  unit="{parsedRecipeIngredient.unit}"
                  ingredientName="{parsedRecipeIngredient.name}"
                  usedIngredientIds="{$list}"
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
    on:submit="{async ({ detail: { values, context } }) => {
      await recipeIngredientRepository.create({
        order: $list.length + 1,
        quantity: values.quantity || null,
        unit: values.unit || null,
        ingredientId: values.ingredientId[0],
        recipeStepId,
      });
      context.reset();
    }}"
  >
    <RecipeIngredientEdit
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
      usedIngredientIds="{$usedIngredientsList || []}"
    />
    <SvelteButton type="submit"
      >{messages.labels.actions.create.format()}</SvelteButton
    >
  </SvelteForm>
{/if}
