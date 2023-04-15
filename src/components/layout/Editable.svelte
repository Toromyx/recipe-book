<!--
@component
This component provides two slots `edit` and `display` and a button to switch between them.

# Events

The `edit` slot content is wrapped in a `<form>` tag with a submit button.
The `edit` event is fired on submitting this form. The detail of this event is the same as the detail of the submit event.
-->

<script>
  import { createEventDispatcher } from "svelte";
  import { messages } from "../../services/translation/en.ts";
  import SvelteButton from "../element/SvelteButton.svelte";
  import SvelteForm from "../element/form/SvelteForm.svelte";

  let edit = false;

  const dispatch = createEventDispatcher();
</script>

{#if edit}
  <SvelteForm
    on:submit="{({ detail }) => {
      dispatch('edit', { ...detail });
      edit = false;
    }}"
  >
    <slot name="edit" />
    <SvelteButton type="submit"
      >{messages.labels.actions.update.format()}</SvelteButton
    >
  </SvelteForm>
{:else}
  <slot name="display" />
{/if}
<SvelteButton on:click="{() => (edit = !edit)}"
  >{edit
    ? messages.labels.actions.cancel.format()
    : messages.labels.actions.edit.format()}</SvelteButton
>
