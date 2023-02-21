<script>
  import { getContext, setContext } from "svelte";
  import { FORM } from "./SvelteForm.svelte";

  export let id;

  /** @type {{[id: number]: any}} */
  const values = {};
  /** @type {{[id: number]: boolean}} */
  const changed = {};
  const formContext = getContext(FORM);

  let setFormValue = () => {};
  let setFormChanged = () => {};
  const fullName = formContext?.name ? `${formContext.name}_${id}` : `${id}`;

  if (formContext) {
    setFormValue = (value) => formContext.setValue(id, value);
    setFormChanged = () => formContext.setChanged(id);
  }

  setContext(FORM, {
    name: fullName,
    setValue: (elementName, value) => {
      values[elementName] = value;
      setFormValue(values);
    },
    setChanged: (elementName) => {
      changed[elementName] = true;
      setFormChanged();
    },
  });
</script>

<slot />
