<script context="module">
  export const FORM = Symbol("Form");
</script>

<script>
  import { createEventDispatcher, setContext } from "svelte";

  /** @type {{[name: string]: any}} */
  const values = {};
  /** @type {{[name: string]: boolean}} */
  const changed = {};
  const dispatch = createEventDispatcher();

  setContext(FORM, {
    setValue: (elementName, value) => {
      values[elementName] = value;
    },
    setChanged: (elementName) => {
      changed[elementName] = true;
    },
  });

  /**
   * @param {Event} event
   */
  function onSubmit(event) {
    dispatch(event.type, { values, changed });
  }
</script>

<form on:submit|preventDefault="{onSubmit}">
  <slot />
</form>
