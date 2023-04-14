<script>
  import { createEventDispatcher, getContext, onDestroy } from "svelte";
  import { FORM } from "./SvelteForm.svelte";

  export let label;
  export let name;
  export let value = "";
  export let placeholder = label;
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
