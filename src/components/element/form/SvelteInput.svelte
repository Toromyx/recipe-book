<script>
  import { Temporal } from "@js-temporal/polyfill";
  import { createEventDispatcher, getContext, tick } from "svelte";
  import { FORM } from "./SvelteForm.svelte";

  export let label;
  export let name;
  export let value = undefined;
  export let type = "text";
  export let placeholder = label;
  export let required = false;
  export let min = undefined;

  export let max = undefined;

  export let step = undefined;

  export let accept = undefined;

  export let multiple = undefined;

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
        case "file":
          if (multiple) {
            return [...eventTarget.files];
          }
          return eventTarget.files.item(0) || undefined;
        default:
          return eventTarget.value;
      }
    })(event.target);
    setChanged();
    void tick().then(() => {
      dispatch(event.type, value);
    });
  }

  function getValueForHtmlElement(value) {
    if (!value || type === "file") {
      return "";
    }
    return value;
  }
</script>

<input
  on:input="{onInputOrChange}"
  on:change="{onInputOrChange}"
  name="{fullName}"
  type="{type}"
  value="{getValueForHtmlElement(value)}"
  placeholder="{placeholder}"
  required="{required}"
  aria-label="{label}"
  min="{min}"
  max="{max}"
  step="{step}"
  accept="{accept}"
  multiple="{multiple}"
/>
