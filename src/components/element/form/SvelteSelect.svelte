<script>
  import { createEventDispatcher, getContext, onDestroy } from "svelte";
  import { messages } from "../../../services/translation/en.ts";
  import { FORM } from "./SvelteForm.svelte";

  /** @type {{value: unknown, label: string}[]} */
  export let options;
  export let label;
  export let name;
  export let placeholder = messages.imperatives.selectPlaceholder.format({
    label,
  });
  export let required = false;
  export let value = undefined;

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
    innerValue = options[event.target.value].value;
    setChanged();
    dispatch(event.type, innerValue);
  }

  onDestroy(() => {
    formContext?.onDestroy(name);
  });
</script>

<select
  on:input="{onInputOrChange}"
  on:change="{onInputOrChange}"
  name="{fullName}"
  required="{required}"
  aria-label="{label}"
>
  <option hidden="{required}" disabled="{required}" value=""
    >{placeholder}</option
  >
  {#each options as option, i}
    <option value="{i}">{option.label}</option>
  {/each}
</select>
