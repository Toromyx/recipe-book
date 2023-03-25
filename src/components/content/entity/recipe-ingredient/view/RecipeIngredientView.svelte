<script>
  import { recipeIngredientRepository } from "../../../../../services/store/repository/recipe-ingredient-repository.ts";
  import { isLoading } from "../../../../../services/util/is-loading.ts";
  import Editable from "../../../../layout/Editable.svelte";
  import IngredientViewName from "../../ingredient/view/IngredientViewName.svelte";
  import RecipeIngredientEdit from "../edit/RecipeIngredientEdit.svelte";

  export let id;

  /** @type {Readable<Loadable<RecipeIngredientInterface>>} */
  let recipeIngredient;

  $: recipeIngredient = recipeIngredientRepository.createStore(id);
  $: quantifiers = [
    $recipeIngredient?.quantity,
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
