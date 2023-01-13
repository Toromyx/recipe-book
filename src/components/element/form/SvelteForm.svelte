<script context="module">
  export const FORM = Symbol("Form");
</script>

<script>
  import { createEventDispatcher, setContext } from "svelte";

  /** @type {{[name: string]: any}} */
  const values = {};
  const dispatch = createEventDispatcher();

  setContext(FORM, {
    setValue: (elementName, value) => {
      values[elementName] = value;
    },
  });

  /**
   * @param {Event} event
   */
  function onSubmit(event) {
    dispatch(event.type, values);
  }
</script>

<form on:submit|preventDefault="{onSubmit}">
  <slot />
</form>
