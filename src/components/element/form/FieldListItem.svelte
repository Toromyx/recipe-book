<!--
@component
This component only creates a new form context for usage as an item in a value array of a field set.
-->

<script>
  import { getContext, onDestroy, setContext } from "svelte";
  import { deleteEntry } from "../../../services/util/delete-entry.ts";
  import { FORM } from "./SvelteForm.svelte";

  /**
   * the index of the item in the value array
   * @type {number}
   */
  export let index;

  /** @type {{[id: number]: unknown}} */
  const values = {};
  /** @type {{[id: number]: boolean}} */
  const changed = {};
  /** @type {{[id: number]: () => void}} */
  const resets = {};
  const formContext = getContext(FORM);

  let setFormValue = () => {};
  let setFormChanged = () => {};
  let formOnDestroy = () => {};
  const fullName = formContext?.name
    ? `${formContext.name}_${index}`
    : `${index}`;

  if (formContext) {
    setFormValue = (value) => formContext.setValue(index, value);
    setFormChanged = () => formContext.setChanged(index);
    formOnDestroy = () => formContext.onDestroy(index);
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
    formContext.registerReset(index, () => {
      context.reset();
    });
  }

  onDestroy(() => {
    formOnDestroy();
  });
</script>

<slot />
