<!--
@component
This component implement a file input form field with a button to open a file dialog.

File dropping is possible.

The value will be a local path.

# Events

The `input` or `change` events are fired when the user sets the value of the field.
-->

<script>
  import { open } from "@tauri-apps/api/dialog";
  import { createEventDispatcher, getContext, onDestroy } from "svelte";
  import { fileDrop } from "../../../services/actions/file-drop.ts";
  import { messages } from "../../../services/translation/en.ts";
  import SvelteButton from "../SvelteButton.svelte";
  import { FORM } from "./SvelteForm.svelte";

  /**
   * the form element label
   * @type {string}
   */
  export let label;
  /**
   * the form element name
   * @type {string}
   */
  export let name;
  /**
   * the form element value
   * @type {string}
   */
  export let value = "";
  /**
   * the form element placeholder
   * @type {string}
   */
  export let placeholder = label;
  /**
   * whether the form element is required or not
   * @type {boolean}
   */
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
