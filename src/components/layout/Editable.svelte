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
    on:submit="{({ detail: { values, changed } }) => {
      dispatch('edit', { values, changed });
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
