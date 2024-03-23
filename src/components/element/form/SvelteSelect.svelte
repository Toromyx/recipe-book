<!--
@component
This component implements a single select form element.

# Events

The `input` or `change` events are fired when the user selects an option.
-->

<script>
  import { createEventDispatcher, getContext, onDestroy } from "svelte";
  import { messages } from "../../../services/translation/en.ts";
  import { FORM } from "./SvelteForm.svelte";

  /**
   * the selectable options
   * @type {unknown[]}
   */
  export let options;
  /**
   * the form element label
   */
  export let label;
  /**
   * the form element name
   */
  export let name;
  /**
   * the form element placeholder
   * @type {string}
   */
  export let placeholder = messages.imperatives.selectPlaceholder.format({
    label,
  });
  /**
   * whether the form element is required or not
   * @type {boolean}
   */
  export let required = false;
  /**
   * the form element value
   * @type {?unknown}
   */
  export let value = undefined;

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
      dispatch("input", innerValue);
      dispatch("change", innerValue);
    });
  }

  $: innerValue = value;
  $: setValue(innerValue);

  function onInputOrChange(event) {
    innerValue = options[event.target.value];
    setChanged();
    dispatch(event.type, innerValue);
  }

  onDestroy(() => {
    formContext?.onDestroy(name);
  });
</script>

<select
  on:input="{onInputOrChange}"
  on:change="{onInputOrChange}"
  name="{fullName}"
  required="{required}"
  aria-label="{label}"
>
  <option hidden="{required}" disabled="{required}" value=""
    >{placeholder}</option
  >
  {#each options as option, i}
    <option value="{i}"><slot option="{option}" /></option>
  {/each}
</select>
