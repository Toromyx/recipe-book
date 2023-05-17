<!--
@component
This component displays the content of recipe ingredient.

The recipe ingredient is editable.
-->

<script>
  import { recipeIngredientRepository } from "../../../../../services/store/repository/recipe-ingredient-repository.ts";
  import { isLoading } from "../../../../../services/util/loadable.ts";
  import Editable from "../../../../layout/Editable.svelte";
  import IngredientViewName from "../../ingredient/view/IngredientViewName.svelte";
  import RecipeIngredientEdit from "../edit/RecipeIngredientEdit.svelte";

  /**
   * the recipe ingredient id
   * @type {number}
   */
  export let id;
  /**  @type {number} */
  export let factor = 1;

  /** @type {Readable<Loadable<RecipeIngredientInterface>>} */
  let recipeIngredient;

  $: recipeIngredient = recipeIngredientRepository.createStore(id);
  $: quantifiers = [
    $recipeIngredient?.quantity * factor,
    $recipeIngredient?.unit,
  ].filter(Boolean);
</script>

{#if !isLoading($recipeIngredient)}
  <Editable
    on:edit="{({ detail: { values, changed } }) => {
      const update = { id };
      if (changed.unit) {
        update.unit = values.unit || null;
      }
      if (changed.quantity) {
        update.quantity = values.quantity || null;
      }
      if (changed.quality) {
        update.quality = values.quality || null;
      }
      if (changed.ingredientId) {
        update.ingredientId = values.ingredientId[0];
      }
      recipeIngredientRepository.update(id, () => update);
    }}"
  >
    <span slot="display"
      >{#each quantifiers as qualifier}{qualifier}&nbsp;{/each}<IngredientViewName
        id="{$recipeIngredient.ingredientId}"
      />{#if $recipeIngredient.quality}{` (${$recipeIngredient.quality})`}{/if}</span
    >
    <RecipeIngredientEdit
      slot="edit"
      quantity="{$recipeIngredient.quantity}"
      unit="{$recipeIngredient.unit}"
      ingredientId="{$recipeIngredient.ingredientId}"
      quality="{$recipeIngredient.quality}"
    />
  </Editable>
{/if}
