<script>
  import { createEventDispatcher, getContext, tick } from "svelte";
  import { FORM } from "./SvelteForm.svelte";
  import { Temporal } from "@js-temporal/polyfill";

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
    value = ((eventTarget) => {
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
