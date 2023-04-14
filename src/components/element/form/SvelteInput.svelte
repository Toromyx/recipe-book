<script>
  import { Temporal } from "@js-temporal/polyfill";
  import { createEventDispatcher, getContext, onDestroy } from "svelte";
  import { FORM } from "./SvelteForm.svelte";

  export let label;
  export let name;
  export let value = undefined;
  /**
   * @type {'text'|'number'|'range'|'date'|'time'|'datetime-local'}
   */
  export let type = "text";
  export let placeholder = label;
  export let required = false;
  export let min = undefined;

  export let max = undefined;

  export let step = "any";

  export let accept = undefined;

  export let multiple = undefined;

  export let list = undefined;

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

  $: innerValue = value || "";
  $: setValue(innerValue);

  function onInputOrChange(event) {
    innerValue = ((eventTarget) => {
      switch (type) {
        case "number":
        case "range":
          return eventTarget.valueAsNumber;
        case "date":
          return Temporal.PlainDate.from(eventTarget.value);
        case "time":
          return Temporal.PlainTime.from(eventTarget.value);
        case "datetime-local":
          return Temporal.PlainDateTime.from(eventTarget.value);
        default:
          return eventTarget.value;
      }
    })(event.target);
    setChanged();
    dispatch(event.type, innerValue);
  }

  onDestroy(() => {
    formContext?.onDestroy(name);
  });
</script>

<input
  on:input="{onInputOrChange}"
  on:change="{onInputOrChange}"
  on:paste
  name="{fullName}"
  type="{type}"
  value="{innerValue}"
  placeholder="{placeholder}"
  required="{required}"
  aria-label="{label}"
  min="{min}"
  max="{max}"
  step="{step}"
  accept="{accept}"
  multiple="{multiple}"
  list="{list}"
/>
