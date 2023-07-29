<!--
@component
This component provides a form to prompt the user to create a list of recipe ingredients.

The list of recipe ingredients is provided as list of parsed recipe ingredient objects.
This component automatically creates the recipe ingredients with the supplied recipe step id.

# Events

The event `done` is emitted either on creation or cancellation. This can be used to clean up the supplied list of recipe ingredients.
-->

<script>
  import { createEventDispatcher } from "svelte";
  import { recipeIngredientRepository } from "../../../../../../services/store/repository/recipe-ingredient-repository.ts";
  import { messages } from "../../../../../../services/translation/en.ts";
  import SvelteButton from "../../../../../element/SvelteButton.svelte";
  import FieldListItem from "../../../../../element/form/FieldListItem.svelte";
  import SvelteFieldset from "../../../../../element/form/SvelteFieldset.svelte";
  import SvelteForm from "../../../../../element/form/SvelteForm.svelte";
  import RecipeIngredientEdit from "../../edit/RecipeIngredientEdit.svelte";

  /**
   * @type {number}
   */
  export let numIngredients;

  /** @type {number} */
  export let recipeStepId;
  /** @type {(ParsedRecipeIngredient & {id?: number})[]} */
  export let parsedRecipeIngredients;
  /** @type {number[]}*/
  export let usedIngredientIds;

  $: ingredientIds = Object.fromEntries(
    Object.entries(parsedRecipeIngredients)
      .map(([key, value]) => {
        if (!value.id) {
          return undefined;
        }
        return [key, value.id];
      })
      .filter(Boolean),
  );

  let dispatch = createEventDispatcher();
</script>

<SvelteForm
  on:submit="{({ detail: { values } }) => {
    for (let i = 0; i < values.ingredients.length; i++) {
      const ingredient = values.ingredients[i];
      recipeIngredientRepository.create({
        order: numIngredients + 1 + i,
        quantity: ingredient.quantity || null,
        unit: ingredient.unit || null,
        quality: ingredient.quality || null,
        ingredientId: ingredient.ingredientId[0],
        recipeStepId,
      });
    }
    dispatch('done');
  }}"
>
  <SvelteFieldset name="ingredients" isList="{true}">
    <ol>
      {#each parsedRecipeIngredients as parsedRecipeIngredient, i (parsedRecipeIngredient)}
        <li>
          <FieldListItem index="{i}">
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
                parsedRecipeIngredients = parsedRecipeIngredients;
              }}"
              quantity="{parsedRecipeIngredient.quantity}"
              unit="{parsedRecipeIngredient.unit}"
              ingredientName="{parsedRecipeIngredient.name}"
              ingredientId="{parsedRecipeIngredient.id}"
              quality="{parsedRecipeIngredient.quality}"
              usedIngredientIds="{[
                ...usedIngredientIds,
                ...Object.entries(ingredientIds)
                  .filter(([key]) => Number(key) !== i)
                  .map(([, value]) => value),
              ]}"
            />
          </FieldListItem>
          <SvelteButton
            on:click="{() => {
              parsedRecipeIngredients.splice(i, 1);
              parsedRecipeIngredients = parsedRecipeIngredients;
            }}"
            confirmation="{true}"
            >{messages.labels.actions.remove.format()}</SvelteButton
          >
        </li>
      {/each}
    </ol>
    <SvelteButton
      on:click="{() => {
        parsedRecipeIngredients = [...parsedRecipeIngredients, { name: '' }];
      }}">{messages.labels.actions.add.format()}</SvelteButton
    >
  </SvelteFieldset>
  <SvelteButton type="submit"
    >{messages.labels.actions.create.format()}</SvelteButton
  >
  <SvelteButton on:click="{() => dispatch('done')}"
    >{messages.labels.actions.cancel.format()}</SvelteButton
  >
</SvelteForm>
