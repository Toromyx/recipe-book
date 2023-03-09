<script>
  import { open } from "@tauri-apps/api/dialog";
  import { createEventDispatcher, getContext, onDestroy, tick } from "svelte";
  import { fileDrop } from "../../../services/actions/file-drop.ts";
  import { messages } from "../../../services/translation/en.ts";
  import SvelteButton from "../SvelteButton.svelte";
  import { FORM } from "./SvelteForm.svelte";

  export let label;
  export let name;
  export let value = "";
  export let placeholder = label;
  export let required = false;

  const formContext = getContext(FORM);
  const dispatch = createEventDispatcher();
  const fullName = formContext?.name ? `${formContext.name}_${name}` : name;

  let input;
  let setValue = () => {};
  let setChanged = () => {};

  if (formContext) {
    setValue = (v) => formContext.setValue(name, v);
    setChanged = () => formContext.setChanged(name);
    formContext.registerReset(name, () => {
      value = "";
    });
  }

  $: setValue(value);

  function onDialogOrFileDrop(userValue) {
    onUserInput(userValue);
  }

  function onInputOrChange(event) {
    onUserInput(event.target.value, event.type);
  }

  function onUserInput(userValue, eventType = "input") {
    value = userValue;
    setChanged();
    void tick().then(() => {
      dispatch(eventType, value);
    });
  }

  onDestroy(() => {
    formContext?.onDestroy(name);
  });
</script>

<span
  use:fileDrop
  on:fileDrop="{({ detail }) => {
    if (detail.length) {
      onDialogOrFileDrop(detail[0]);
    }
  }}"
>
  <SvelteButton
    on:click="{async () => {
      const selected = await open();
      if (selected) {
        onDialogOrFileDrop(selected);
      }
    }}">{messages.labels.actions.file.open.format()}</SvelteButton
  >
  <input
    bind:this="{input}"
    on:input="{onInputOrChange}"
    on:change="{onInputOrChange}"
    name="{fullName}"
    value="{value}"
    placeholder="{placeholder}"
    required="{required}"
    aria-label="{label}"
  />
</span>
