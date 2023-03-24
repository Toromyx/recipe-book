<script>
  import { getContext, onDestroy, setContext } from "svelte";
  import { deleteEntry } from "../../../services/util/delete-entry.ts";
  import { FORM } from "./SvelteForm.svelte";

  export let id;

  /** @type {{[id: number]: any}} */
  const values = {};
  /** @type {{[id: number]: boolean}} */
  const changed = {};
  /** @type {{[id: number]: () => void}} */
  const resets = {};
  const formContext = getContext(FORM);

  let setFormValue = () => {};
  let setFormChanged = () => {};
  let formOnDestroy = () => {};
  const fullName = formContext?.name ? `${formContext.name}_${id}` : `${id}`;

  if (formContext) {
    setFormValue = (value) => formContext.setValue(id, value);
    setFormChanged = () => formContext.setChanged(id);
    formOnDestroy = () => formContext.onDestroy(id);
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
    formContext.registerReset(id, () => {
      context.reset();
    });
  }

  onDestroy(() => {
    formOnDestroy();
  });
</script>

<slot />
