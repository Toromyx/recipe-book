<script context="module">
  export const FORM = Symbol("Form");
</script>

<script>
  import { createEventDispatcher, setContext } from "svelte";

  /** @type {{[name: string]: any}} */
  const values = {};
  /** @type {{[name: string]: boolean}} */
  const changed = {};
  /** @type {{[name: string]: () => void}} */
  const resets = {};
  const dispatch = createEventDispatcher();

  const context = setContext(FORM, {
    setValue: (elementName, value) => {
      values[elementName] = value;
    },
    setChanged: (elementName) => {
      changed[elementName] = true;
    },
    registerReset: (elementName, reset) => {
      resets[elementName] = () => reset();
    },
    reset: () => {
      for (const reset of Object.values(resets)) {
        reset();
      }
    },
    onDestroy: (elementName) => {
      delete values[elementName];
      delete changed[elementName];
      delete resets[elementName];
    },
  });

  /**
   * @param {SubmitEvent} event
   */
  function onSubmit(event) {
    dispatch(event.type, { values, changed, context });
  }

  function onReset() {
    context.reset();
  }
</script>

<form on:submit|preventDefault="{onSubmit}" on:reset="{onReset}">
  <slot />
</form>
