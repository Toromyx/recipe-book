<!--
@component
This component implements a standard form input of different types:
- text
- number
- range
- date
- time
- datetime

The value of the form element is typed accordingly.

# Events

The `input` or `change` events are fired when the user sets the value of the field.
-->

<script>
  import { Temporal } from "@js-temporal/polyfill";
  import { createEventDispatcher, getContext, onDestroy } from "svelte";
  import { setCustomValidity } from "../../../services/util/validity.ts";
  import { FORM } from "./SvelteForm.svelte";

  /**
   * @typedef {string|number|Temporal.PlainDate|Temporal.PlainTime|Temporal.PlainDateTime|undefined} ValueType
   */

  /**
   * the form element label
   * @type {string}
   */
  export let label;
  /**
   * the form element name
   * @type {string}
   */
  export let name;
  /**
   * the form element value
   * @type ValueType
   */
  export let value = undefined;
  /**
   * the form element type
   * @type {'text'|'number'|'range'|'date'|'time'|'datetime-local'}
   */
  export let type = "text";
  /**
   * the form element placeholder
   * @type {string}
   */
  export let placeholder = label;
  /**
   * whether the form element is required or not
   * @type {boolean}
   */
  export let required = false;
  /**
   * the minimum value of the input
   * @type {?number}
   */
  export let min = undefined;
  /**
   * the maximum value of the input
   * @type {?number}
   */
  export let max = undefined;
  /**
   * the step size of the input
   * @type {'any'|number}
   */
  export let step = "any";
  /**
   * the id of a datalist for autocomplete suggestions
   * @type {?string}
   */
  export let list = undefined;
  /**
   * The validator functions to run on every change of the value.
   * @type {Array<(value: ValueType) => ?string>}
   */
  export let validators = [];

  const formContext = getContext(FORM);
  const dispatch = createEventDispatcher();
  const fullName = formContext?.name ? `${formContext.name}_${name}` : name;

  /**
   * @type {?HTMLInputElement}
   */
  let input;
  let innerValue;
  let setValue = () => {};
  let setChanged = () => {};

  if (formContext) {
    setValue = (v) => formContext.setValue(name, v);
    setChanged = () => formContext.setChanged(name);
    formContext.registerReset(name, () => {
      innerValue = undefined;
      dispatch("input", innerValue);
      dispatch("change", innerValue);
    });
  }

  $: innerValue = value;
  $: setValue(innerValue);
  $: setCustomValidity(input, innerValue, ...validators);
  $: htmlValue = (() => {
    switch (type) {
      case "number":
      case "range":
        return !isNaN(innerValue) ? innerValue : "";
      default:
        return innerValue ?? "";
    }
  })();

  /**
   * Set and report the custom validity on the HTML input.
   *
   * @param {string} error
   * @see HTMLInputElement.setCustomValidity
   * @see HTMLInputElement.reportValidity
   */
  export function setAndReportCustomValidity(error) {
    input?.setCustomValidity(error);
    input?.reportValidity();
  }

  function onInputOrChange(event) {
    innerValue = ((eventTarget) => {
      switch (type) {
        case "number":
        case "range":
          return eventTarget.valueAsNumber;
        case "date":
          return Temporal.PlainDate.from(eventTarget.value);
        case "time":
          return Temporal.PlainTime.from(eventTarget.value);
        case "datetime-local":
          return Temporal.PlainDateTime.from(eventTarget.value);
        default:
          return eventTarget.value;
      }
    })(event.target);
    setChanged();
    dispatch(event.type, innerValue);
  }

  onDestroy(() => {
    formContext?.onDestroy(name);
  });
</script>

<input
  bind:this="{input}"
  on:input="{onInputOrChange}"
  on:change="{onInputOrChange}"
  on:paste
  name="{fullName}"
  type="{type}"
  value="{htmlValue}"
  placeholder="{placeholder}"
  required="{required}"
  aria-label="{label}"
  min="{min}"
  max="{max}"
  step="{step}"
  list="{list}"
/>
