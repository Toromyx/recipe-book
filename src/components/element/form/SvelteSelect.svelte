<script>
  import { createEventDispatcher, getContext, tick } from "svelte";
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

  let setValue = () => {};
  let setChanged = () => {};

  if (formContext) {
    setValue = (v) => formContext.setValue(name, v);
    setChanged = () => formContext.setChanged(name);
  }

  $: setValue(value);

  function onInputOrChange(event) {
    value = options[event.target.value].value;
    setChanged();
    void tick().then(() => {
      dispatch(event.type, value);
    });
  }
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
