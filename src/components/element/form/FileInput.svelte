<script>
  import { open } from "@tauri-apps/api/dialog";
  import { createEventDispatcher, getContext, onDestroy } from "svelte";
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

  let innerValue;
  let input;
  let setValue = () => {};
  let setChanged = () => {};

  if (formContext) {
    setValue = (v) => formContext.setValue(name, v);
    setChanged = () => formContext.setChanged(name);
    formContext.registerReset(name, () => {
      innerValue = "";
    });
  }

  $: innerValue = value;
  $: setValue(innerValue);

  function onDialogOrFileDrop(userValue) {
    onUserInput(userValue);
  }

  function onInputOrChange(event) {
    onUserInput(event.target.value, event.type);
  }

  function onUserInput(userValue, eventType = "input") {
    innerValue = userValue;
    setChanged();
    dispatch(eventType, innerValue);
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
    value="{innerValue}"
    placeholder="{placeholder}"
    required="{required}"
    aria-label="{label}"
  />
</span>
