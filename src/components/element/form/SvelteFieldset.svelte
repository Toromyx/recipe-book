<!--
@component
This component creates a form context for a structured subset of form elements.

This structured subset can be an object or array.
-->

<script>
  import { getContext, onDestroy, setContext } from "svelte";
  import { deleteEntry } from "../../../services/util/delete-entry.ts";
  import { FORM } from "./SvelteForm.svelte";

  /**
   * the form element subset name
   * @type {string}
   */
  export let name;
  /**
   * the form element subset label
   * @type {?string}
   */
  export let label = undefined;
  /**
   * whether the form element subset is an object or array
   * @type {boolean}
   */
  export let isList = false;

  /** @type {{[name: string]: unknown}|unknown[]} */
  const values = isList ? [] : {};
  /** @type {{[name: string]: boolean}} */
  const changed = {};
  /** @type {{[name: string]: () => void}} */
  const resets = {};
  const formContext = getContext(FORM);

  let setFormValue = () => {};
  let setFormChanged = () => {};
  let formOnDestroy = () => {};
  const fullName = formContext?.name ? `${formContext.name}_${name}` : name;

  if (formContext) {
    setFormValue = (value) => formContext.setValue(name, value);
    setFormChanged = () => formContext.setChanged(name);
    formOnDestroy = () => formContext.onDestroy(name);
  }

  const context = setContext(FORM, {
    name: fullName,
    setValue: (elementName, value) => {
      values[elementName] = value;
      setFormValue(values);
    },
    setChanged: (elementName) => {
      changed[elementName] = true;
      setFormChanged();
    },
    registerReset: (elementName, reset) => {
      resets[elementName] = () => reset();
    },
    reset: () => {
      for (const reset of Object.values(resets)) {
        reset();
      }
    },
    onDestroy: (elementName) => {
      deleteEntry(values, elementName);
      deleteEntry(changed, elementName);
      deleteEntry(resets, elementName);
    },
  });

  if (formContext) {
    formContext.registerReset(name, () => {
      context.reset();
    });
  }

  onDestroy(() => {
    formOnDestroy();
  });
</script>

<fieldset>
  {#if label}
    <legend>{label}</legend>
  {/if}
  <slot />
</fieldset>
