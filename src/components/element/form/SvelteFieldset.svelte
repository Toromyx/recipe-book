<script>
  import { getContext, setContext } from "svelte";
  import { FORM } from "./SvelteForm.svelte";

  export let name;
  export let label;

  /** @type {{[name: string]: any}} */
  const values = {};
  /** @type {{[name: string]: boolean}} */
  const changed = {};
  const formContext = getContext(FORM);

  let setFormValue = () => {};
  let setFormChanged = () => {};
  const fullName = formContext?.name ? `${formContext.name}_${name}` : name;

  if (formContext) {
    setFormValue = (value) => formContext.setValue(name, value);
    setFormChanged = () => formContext.setChanged(name);
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

<fieldset>
  <legend>{label}</legend>
  <slot />
</fieldset>
