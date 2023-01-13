<script>
  import { createEventDispatcher, getContext, tick } from "svelte";
  import { FORM } from "./SvelteForm.svelte";

  export let label;
  export let name;
  export let value = "";
  export let type = "text";
  export let placeholder = label;
  export let required = false;
  export let min = undefined;

  export let max = undefined;

  export let step = undefined;

  const formContext = getContext(FORM);
  const dispatch = createEventDispatcher();
  const fullName = formContext?.name ? `${formContext.name}_${name}` : name;

  let setValue = () => {};

  if (formContext) {
    setValue = (v) => formContext.setValue(name, v);
  }

  $: setValue(value);

  function onInputOrChange(event) {
    value = event.target.value;
    void tick().then(() => {
      dispatch(event.type, value);
    });
  }
</script>

<input
  on:input="{onInputOrChange}"
  on:change="{onInputOrChange}"
  value="{value}"
  name="{fullName}"
  type="{type}"
  placeholder="{placeholder}"
  required="{required}"
  aria-label="{label}"
  min="{min}"
  max="{max}"
  step="{step}"
/>
