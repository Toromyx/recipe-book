<!--
@component
This component implements a button to be used everywhere a button is needed.

A confirmation dialog can optionally be enabled for this component.

# Events

The `click` event is dispatched when the button is clicked or the dialog confirmed.
-->

<script>
  import { createEventDispatcher } from "svelte";
  import ConfirmDialog from "../layout/ConfirmDialog.svelte";

  /**
   * The button type
   * @type {'button'|'submit'|'reset'}
   */
  export let type = "button";
  /**
   * Whether the button is disabled
   * @type {boolean}
   */
  export let disabled = false;
  /**
   * The button value to be used in forms
   * @type {string}
   */
  export let value = "";
  /**
   * The button id
   * @type {?string}
   */
  export let id = undefined;
  /**
   * Whether to use a confirmation dialog when pressing this button
   * @type {boolean}
   */
  export let confirmation = false;
  /**
   * The question to ask the user in the confirmation dialog
   * @type {?string}
   */
  export let confirmationQuestion = undefined;
  /**
   * The cancellation text to show to the user in the confirmation dialog
   * @type {?string}
   */
  export let confirmationCancel = undefined;
  /**
   * The confirmation text to show to the user in the confirmation dialog
   * @type {?string}
   */
  export let confirmationConfirm = undefined;

  let confirmDialog;
  let dispatch = createEventDispatcher();

  function onClick() {
    if (confirmation) {
      confirmDialog.show();
      return;
    }

    dispatch("click");
  }
</script>

<button
  on:click="{onClick}"
  id="{id}"
  type="{type}"
  disabled="{disabled}"
  value="{value}"
>
  <slot />
</button>
{#if confirmation}
  <ConfirmDialog
    bind:this="{confirmDialog}"
    on:confirm="{() => {
      dispatch('click');
    }}"
    question="{confirmationQuestion}"
    cancel="{confirmationCancel}"
    confirm="{confirmationConfirm}"
  />
{/if}
