<!--
@component
This component implements a confirm dialog with a `<dialog>` tag.

Since the `<dialog>` tag contains a `<form>` tag, it is moved to the document body to avoid form nesting.

# Events

The `confirm` event is fired when the user confirms the dialog, the `cancel` event is fired when the user cancels the dialog.
These events are not fired when the dialog is closed programmatically.
-->

<script>
  import { createEventDispatcher } from "svelte";
  import { portal } from "../../services/actions/portal.ts";
  import { messages } from "../../services/translation/en.ts";
  import SvelteButton from "../element/SvelteButton.svelte";

  /**
   * The question to ask the user
   * @type {string}
   */
  export let question = messages.questions.confirmation.format();
  /**
   * The cancellation text to show to the user
   * @type {string}
   */
  export let cancel = messages.labels.actions.cancel.format();
  /**
   * The confirmation text to show to the user
   * @type {string}
   */
  export let confirm = messages.labels.actions.confirm.format();

  /** @type {HTMLDialogElement|undefined} */
  let dialog;
  let dispatch = createEventDispatcher();

  /**
   * Close the confirmation dialog.
   */
  export function close() {
    dialog?.close();
  }

  /**
   * Show the confirmation dialog.
   */
  export function show() {
    dialog?.showModal();
  }

  function onClose() {
    if (dialog?.returnValue === "confirm") {
      dispatch("confirm");
    } else {
      dispatch("cancel");
    }
  }
</script>

<div use:portal="{document.body}">
  <dialog bind:this="{dialog}" on:close="{onClose}">
    <p>{question}</p>
    <form method="dialog">
      <SvelteButton type="submit" value="cancel">{cancel}</SvelteButton>
      <SvelteButton type="submit" value="confirm">{confirm}</SvelteButton>
    </form>
  </dialog>
</div>
