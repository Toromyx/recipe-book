<script>
  import { createEventDispatcher, getContext, tick } from "svelte";
  import { FORM } from "./SvelteForm.svelte";

  export let label;
  export let name;
  export let value = "";
  export let placeholder = label;
  export let required = false;

  const formContext = getContext(FORM);
  const dispatch = createEventDispatcher();
  const fullName = formContext?.name ? `${formContext.name}_${name}` : name;

  let setValue = () => {};
  let setChanged = () => {};

  if (formContext) {
    setValue = (v) => formContext.setValue(name, v);
    setChanged = () => formContext.setChanged(name);
  }

  $: setValue(value);

  function onInputOrChange(event) {
    value = event.target.value;
    setChanged();
    void tick().then(() => {
      dispatch(event.type, value);
    });
  }
</script>

<textarea
  on:input="{onInputOrChange}"
  on:change="{onInputOrChange}"
  on:paste
  name="{fullName}"
  placeholder="{placeholder}"
  required="{required}"
  aria-label="{label}">{value}</textarea
>
