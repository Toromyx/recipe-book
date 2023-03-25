<script>
  import { createEventDispatcher } from "svelte";
  import { portal } from "../../services/actions/portal.ts";
  import { messages } from "../../services/translation/en.ts";
  import SvelteButton from "../element/SvelteButton.svelte";

  export let question = messages.questions.confirmation.format();
  export let cancel = messages.labels.actions.cancel.format();
  export let confirm = messages.labels.actions.confirm.format();

  /** @type {HTMLDialogElement|undefined} */
  let dialog;
  let dispatch = createEventDispatcher();

  export function close() {
    dialog?.close();
  }

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
