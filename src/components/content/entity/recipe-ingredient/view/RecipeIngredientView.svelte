<script>
  import { recipeIngredientRepository } from "../../../../../services/repository/recipe-ingredient-repository.ts";
  import { isLoading } from "../../../../../services/util/is-loading.ts";
  import Editable from "../../../../layout/Editable.svelte";
  import IngredientViewName from "../../ingredient/view/IngredientViewName.svelte";
  import RecipeIngredientEdit from "../edit/RecipeIngredientEdit.svelte";

  export let id;

  /** @type {Readable<Loadable<RecipeIngredientInterface>>} */
  let recipeIngredient;
  let qualifiers;

  $: recipeIngredient = recipeIngredientRepository.createStore(id);
  $: qualifiers = [$recipeIngredient?.quantity, $recipeIngredient?.unit].filter(
    Boolean,
  );
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
      if (changed.ingredientId) {
        update.ingredientId = values.ingredientId[0];
      }
      recipeIngredientRepository.update(id, () => update);
    }}"
  >
    <span slot="display"
      >{#each qualifiers as qualifier}{qualifier}&nbsp;{/each}<IngredientViewName
        id="{$recipeIngredient.ingredientId}"
      /></span
    >
    <RecipeIngredientEdit
      slot="edit"
      quantity="{$recipeIngredient.quantity}"
      unit="{$recipeIngredient.unit}"
      ingredientId="{$recipeIngredient.ingredientId}"
    />
  </Editable>
{/if}
