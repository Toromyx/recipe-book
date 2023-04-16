<!--
@component
This component implement a form element text area.

# Events

The `input` or `change` events are fired when the user changes the value.
-->

<script>
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
   * @type {string}
   */
  export let value = "";
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
    innerValue = event.target.value;
    setChanged();
    dispatch(event.type, innerValue);
  }

  onDestroy(() => {
    formContext?.onDestroy(name);
  });
</script>

<textarea
  on:input="{onInputOrChange}"
  on:change="{onInputOrChange}"
  on:paste
  name="{fullName}"
  placeholder="{placeholder}"
  required="{required}"
  aria-label="{label}">{innerValue}</textarea
>
