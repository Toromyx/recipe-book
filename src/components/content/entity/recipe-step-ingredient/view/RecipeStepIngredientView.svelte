<!--
@component
This component displays the content of recipe step ingredient.

The recipe step ingredient is editable.
It also takes care of the unit conversion.
-->

<script>
  import { invoke } from "../../../../../services/command/client.ts";
  import { Command } from "../../../../../services/command/command.ts";
  import { recipeStepIngredientRepository } from "../../../../../services/store/repository/recipe-step-ingredient-repository.ts";
  import { unitNameRepository } from "../../../../../services/store/repository/unit-name-repository.ts";
  import { messages } from "../../../../../services/translation/en.ts";
  import {
    isLoaded,
    whenLoadedValue,
  } from "../../../../../services/util/loadable.ts";
  import { Unit } from "../../../../../types/entity/unit-name-interface.ts";
  import Editable from "../../../../layout/Editable.svelte";
  import IngredientViewName from "../../ingredient/view/IngredientViewName.svelte";
  import RecipeStepIngredientEdit from "../edit/RecipeStepIngredientEdit.svelte";

  /**
   * the recipe step ingredient id
   * @type {number}
   */
  export let id;
  /**  @type {number} */
  export let factor = 1;

  const quantityNumberFormat = new Intl.NumberFormat([], {
    maximumSignificantDigits: 3,
  });

  /** @type {Readable<Loadable<RecipeStepIngredientInterface>>} */
  let recipeStepIngredient;
  /** @type {Readable<Loadable<UnitNameInterface> | null> } */
  let unitName;
  /** @type {Loadable<number | null>} */
  let quantity;
  /** @type {Loadable<number | null>} */
  let factoredQuantity;
  /** @type {Loadable<string | null>} */
  let formattedQuantity;
  /** @type {Loadable<string | null>} */
  let unit;
  /** @type {Loadable<string | null>} */
  let formattedUnit;
  /** @type {ConvertedValue[]} */
  let convertedValues = [];
  /** @type {ConvertedValue[]} */
  let sortedConvertedValues;
  /** @type {ConvertedValue | undefined} */
  let mostReadableConvertedValue;

  $: recipeStepIngredient = recipeStepIngredientRepository.createStore(id);
  $: quantity = whenLoadedValue(
    $recipeStepIngredient,
    (recipeStepIngredient) => recipeStepIngredient.quantity,
  );
  $: safeFactor = !isNaN(factor) ? factor : 1;
  $: factoredQuantity = whenLoadedValue(quantity, (quantity) =>
    quantity !== null ? quantity * safeFactor : null,
  );
  $: unit = whenLoadedValue(
    $recipeStepIngredient,
    (recipeStepIngredient) => recipeStepIngredient.unit,
  );
  $: unitName = whenLoadedValue(unit, (unit) =>
    unit ? unitNameRepository.createStore(unit) : undefined,
  );
  $: whenLoadedValue($unitName, (unitName) => {
    if (!unitName) {
      return;
    }
    void invoke(Command.UNIT_CONVERT, {
      value: factoredQuantity,
      unit: unitName.unit,
    }).then((conversion) => (convertedValues = conversion.values));
  });
  $: sortedConvertedValues = convertedValues.sort((a, b) =>
    compareReadability(a.value, b.value),
  );
  $: mostReadableConvertedValue = sortedConvertedValues[0];
  $: formattedQuantity = mostReadableConvertedValue
    ? quantityNumberFormat.format(mostReadableConvertedValue.value)
    : whenLoadedValue(factoredQuantity, (factoredQuantity) =>
        factoredQuantity !== null
          ? quantityNumberFormat.format(factoredQuantity)
          : null,
      );
  $: formattedUnit = mostReadableConvertedValue
    ? translateUnit(
        mostReadableConvertedValue.unit,
        mostReadableConvertedValue.value,
      )
    : unit;
  $: quantifiers = [formattedQuantity, formattedUnit]
    .filter(Boolean)
    .join("\xa0");
  $: originalQuantifiers = [quantity, unit].filter(Boolean).join("\xa0");

  /**
   * @param {Unit} unit
   * @param {number} value
   */
  function translateUnit(unit, value) {
    switch (unit) {
      case Unit.MASS_KILOGRAM:
        return messages.units.kilogram.format({ value });
      case Unit.MASS_GRAM:
        return messages.units.gram.format({ value });
      case Unit.MASS_POUND:
        return messages.units.pound.format({ value });
      case Unit.VOLUME_LITRE:
        return messages.units.litre.format({ value });
      case Unit.VOLUME_MILLILITRE:
        return messages.units.millilitre.format({ value });
      case Unit.VOLUME_US_CUP:
        return messages.units.usCup.format({ value });
    }
  }

  /**
   * Compare two values, with the goal of sorting them in the following way:
   *
   * [
   *  1/-1,
   *  ...[<all numbers with absolute values larger than one in ascending order>],
   *  ...[<all numbers with absolute value smaller than one in descending order>],
   *  0
   * ]
   */
  function compareReadability(valueA, valueB) {
    const [absA, absB] = [valueA, valueB].map(Math.abs);
    if (absA < 1) {
      if (absB >= 1) {
        return 1;
      }
      return absB - absA;
    }
    if (absB < 1) {
      return -1;
    }
    return absA - absB;
  }
</script>

{#if isLoaded($recipeStepIngredient)}
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
      recipeStepIngredientRepository.update(id, () => update);
    }}"
  >
    <span
      slot="display"
      title="{originalQuantifiers
        ? `${originalQuantifiers}\xa0⨉\xa0${safeFactor}`
        : undefined}"
      >{#if quantifiers}{quantifiers}&nbsp;{/if}<IngredientViewName
        id="{$recipeStepIngredient.ingredientId}"
      />{#if $recipeStepIngredient.quality}{` (${$recipeStepIngredient.quality})`}{/if}</span
    >
    <RecipeStepIngredientEdit
      slot="edit"
      quantity="{$recipeStepIngredient.quantity}"
      unit="{$recipeStepIngredient.unit}"
      ingredientId="{$recipeStepIngredient.ingredientId}"
      quality="{$recipeStepIngredient.quality}"
    />
  </Editable>
{/if}
