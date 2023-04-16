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
  import { FORM } from "./SvelteForm.svelte";

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
   * @type {string|number|Temporal.PlainDate|Temporal.PlainTime|Temporal.PlainDateTime|undefined}
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

  const formContext = getContext(FORM);
  const dispatch = createEventDispatcher();
  const fullName = formContext?.name ? `${formContext.name}_${name}` : name;

  let innerValue;
  let setValue = () => {};
  let setChanged = () => {};

  if (formContext) {
    setValue = (v) => formContext.setValue(name, v);
    setChanged = () => formContext.setChanged(name);
    formContext.registerReset(name, () => {
      innerValue = undefined;
    });
  }

  $: innerValue = value;
  $: setValue(innerValue);
  $: htmlValue = innerValue || "";

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
