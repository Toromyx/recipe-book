<script>
  import { getContext, setContext } from "svelte";
  import { FORM } from "./SvelteForm.svelte";

  export let name;
  export let label;

  /** @type {{[name: string]: any}} */
  const values = {};
  const formContext = getContext(FORM);

  let setFormValue = () => {};
  const fullName = formContext?.name ? `${formContext.name}_${name}` : name;

  if (formContext) {
    setFormValue = (value) => formContext.setValue(name, value);
  }

  setContext(FORM, {
    name: fullName,
    setValue: (elementName, value) => {
      values[elementName] = value;
      setFormValue(values);
    },
  });
</script>

<fieldset>
  <legend>{label}</legend>
  <slot />
</fieldset>
