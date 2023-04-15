<!--
@component
This component implements an custom `<form>` tag.

Each component registers a form context.
This context provides methods to manage the form elements:
- their values
- whether they have been changed
- registering their reset function
- a method to reset the whole form
- deleting their data from the context

A form element is always identified by its name.

# Events

The `submit` event is fired when the form is submitted. The details contain the values, their changed state and the complete form context.
-->

<script context="module">
  export const FORM = Symbol("Form");
</script>

<script>
  import { createEventDispatcher, setContext } from "svelte";
  import { deleteEntry } from "../../../services/util/delete-entry.ts";

  /** @type {{[name: string]: unknown}} */
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
      deleteEntry(values, elementName);
      deleteEntry(changed, elementName);
      deleteEntry(resets, elementName);
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
