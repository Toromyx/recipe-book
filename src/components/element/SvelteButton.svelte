<script>
  import { createEventDispatcher } from "svelte";
  import ConfirmDailog from "../layout/ConfirmDailog.svelte";

  /**
   * @type {'button'|'submit'|'reset'}
   */
  export let type = "button";
  export let disabled = false;
  export let value = "";
  export let confirmation = false;
  export let confirmationQuestion = undefined;
  export let confirmationCancel = undefined;
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
  type="{type}"
  disabled="{disabled}"
  value="{value}"
>
  <slot />
</button>
{#if confirmation}
  <ConfirmDailog
    bind:this="{confirmDialog}"
    on:confirm="{() => {
      dispatch('click');
    }}"
    question="{confirmationQuestion}"
    cancel="{confirmationCancel}"
    confirm="{confirmationConfirm}"
  />
{/if}
