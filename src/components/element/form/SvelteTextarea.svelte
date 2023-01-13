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

<textarea
  on:input="{onInputOrChange}"
  on:change="{onInputOrChange}"
  name="{fullName}"
  placeholder="{placeholder}"
  required="{required}"
  aria-label="{label}">{value}</textarea
>
