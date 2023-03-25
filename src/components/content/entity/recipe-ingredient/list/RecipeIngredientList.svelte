<script>
  import {
    parseHtml,
    parseText,
  } from "../../../../../services/parser/recipe-ingredient-parser.ts";
  import { ingredientRepository } from "../../../../../services/store/repository/ingredient-repository.ts";
  import { recipeIngredientRepository } from "../../../../../services/store/repository/recipe-ingredient-repository.ts";
  import { unitList } from "../../../../../services/store/unit-list.ts";
  import { messages } from "../../../../../services/translation/en.ts";
  import {
    whenLoadingDefault,
    isLoading,
  } from "../../../../../services/util/is-loading.ts";
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
  /** @type {(ParsedRecipeIngredient & {id?: number})[]} */
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
  $: pastedIngredientIds = pastedParsedRecipeIngredients
    .map((ingredient) => ingredient.id)
    .filter(Boolean);
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
            quality: ingredient.quality || null,
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
                  on:edit="{({
                    detail: {
                      quantity,
                      unit,
                      ingredientName,
                      ingredientId,
                      quality,
                    },
                  }) => {
                    parsedRecipeIngredient.quantity = quantity || undefined;
                    parsedRecipeIngredient.unit = unit || undefined;
                    parsedRecipeIngredient.name = ingredientName;
                    parsedRecipeIngredient.id = ingredientId;
                    parsedRecipeIngredient.quality = quality;
                    pastedParsedRecipeIngredients =
                      pastedParsedRecipeIngredients;
                  }}"
                  quantity="{parsedRecipeIngredient.quantity}"
                  unit="{parsedRecipeIngredient.unit}"
                  ingredientName="{parsedRecipeIngredient.name}"
                  ingredientId="{parsedRecipeIngredient.id}"
                  quality="{parsedRecipeIngredient.quality}"
                  usedIngredientIds="{[
                    ...$usedIngredientsList,
                    ...pastedIngredientIds,
                  ]}"
                />
              </FieldListItem>
              <SvelteButton
                on:click="{() => {
                  pastedParsedRecipeIngredients.splice(i, 1);
                  pastedParsedRecipeIngredients = pastedParsedRecipeIngredients;
                }}"
                confirmation="{true}"
                >{messages.labels.actions.remove.format()}</SvelteButton
              >
            </li>
          {/each}
        </ol>
        <SvelteButton
          on:click="{() => {
            pastedParsedRecipeIngredients = [
              ...pastedParsedRecipeIngredients,
              { name: '' },
            ];
          }}">{messages.labels.actions.add.format()}</SvelteButton
        >
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
        quality: values.quality || null,
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
          pastedParsedRecipeIngredients = parseHtml(
            html,
            whenLoadingDefault($unitList, []),
          );
          return;
        }
        const text = e.clipboardData.getData('text/text');
        pastedParsedRecipeIngredients = parseText(
          text,
          whenLoadingDefault($unitList, []),
        );
      }}"
      usedIngredientIds="{$usedIngredientsList || []}"
    />
    <SvelteButton type="submit"
      >{messages.labels.actions.create.format()}</SvelteButton
    >
  </SvelteForm>
{/if}
