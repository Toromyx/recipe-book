<!--
@component
This component implements autocomplete functionality in a custom select field, allowing multiple values.

It integrates with the form context to write its value there under its specified name.
Each value can be selected only once.
Possible values are computed via a given callback function which receives the user input as parameter.

# Slots

This component's slot implements the display of a single value. It is given the value as `item`.

# Events

The `input` and `change` events are fired when the user input is changed. Their detail is the user input.

The `select` event is fired when a value is selected or deselected. Its detail is the value array.
-->

<script>
  import { createEventDispatcher, getContext, onDestroy } from "svelte";
  import { messages } from "../../../services/translation/en.ts";
  import { debounce } from "../../../services/util/debounce.ts";
  import { setCustomValidity } from "../../../services/util/validity.ts";
  import SvelteButton from "../SvelteButton.svelte";
  import { FORM } from "./SvelteForm.svelte";

  /**
   * @typedef {unknown} ValueType
   */

  /**
   * The form element label
   * @type {string}
   */
  export let label;
  /**
   * The form element name
   * @type {string}
   */
  export let name;
  /**
   * A function to get possible values from user input
   * @type {(string) => Promise<unknown[]>}
   */
  export let callback;
  /**
   * This is the function to create a new value from user input.
   *
   * The option to create a new value is only presented to the user when this property is defined.
   * @type {(string) => Promise<void>|undefined}
   */
  export let createCallback = undefined;
  /**
   * The value array
   * @type {ValueType[]}
   */
  export let value = [];
  /**
   * The user input
   * @type {string}
   */
  export let userInput = "";
  /**
   * The values which are not allowed to be selected by the user
   * @type {ValueType[]}
   */
  export let excludedValues = [];
  /**
   * The form element placeholder
   * @type {string}
   */
  export let placeholder = label;
  /**
   * The minimum number of value items
   * @type {number}
   */
  export let min = 0;
  /**
   * The maximum number of value items
   * @type {number}
   */
  export let max = 1;
  /**
   * The maximum number of results to display to the user
   * @type {number}
   */
  export let maxResults = 10;
  /**
   * The amount of milliseconds to debounce the result callback
   * @type {number}
   */
  export let debounceWait = 200;

  const formContext = getContext(FORM);
  const dispatch = createEventDispatcher();
  const fullName = formContext?.name ? `${formContext.name}_${name}` : name;
  const getResults = debounce(async (userInput) => {
    if (!userInput) {
      return;
    }
    results = await callback(userInput);
  }, debounceWait);
  const createAndSelect = async () => {
    const createValue = await createCallback(innerUserInput);
    if (createValue) {
      select(createValue);
    }
    results = await callback(innerUserInput);
  };

  let innerValue;
  let innerUserInput;
  let setValue = () => {};
  let setChanged = () => {};
  /** @type {unknown[]} */
  let results = [];
  let hiddenInput;

  if (formContext) {
    setValue = (v) => formContext.setValue(name, v);
    setChanged = () => formContext.setChanged(name);
    formContext.registerReset(name, () => {
      innerValue = [];
      dispatch("select", innerValue);
      innerUserInput = "";
      dispatch("input", innerUserInput);
      dispatch("change", innerUserInput);
    });
  }

  $: innerValue = value;
  $: setValue(innerValue);
  $: innerUserInput = userInput;
  $: getResults(innerUserInput);
  $: filteredResults = results.filter((result) => !innerValue.includes(result));
  $: truncatedResults = filteredResults.slice(0, maxResults);
  $: {
    if (hiddenInput) {
      setCustomValidity(
        hiddenInput,
        innerValue,
        ...[
          (value) =>
            value.length < min
              ? messages.validity.autocomplete.min.format({ min })
              : undefined,
          (value) =>
            value.length > max
              ? messages.validity.autocomplete.max.format({ max })
              : undefined,
          (value) => {
            for (const item of value) {
              if (excludedValues.includes(item)) {
                return messages.validity.autocomplete.includesExcluded.format();
              }
            }
          },
        ],
      );
    }
  }

  function onInputOrChange(event) {
    innerUserInput = event.target.value;
    dispatch(event.type, innerUserInput);
  }

  function select(item) {
    innerValue = [...innerValue, item];
    setChanged();
    dispatch("select", innerValue);
  }

  function deselect(item) {
    innerValue = innerValue.filter((valueItem) => valueItem !== item);
    setChanged();
    dispatch("select", innerValue);
  }

  onDestroy(() => {
    formContext.onDestroy(name);
  });
</script>

<div class="autocomplete">
  <span
    >{#each innerValue as item}<span
        ><slot item="{item}" /><SvelteButton on:click="{() => deselect(item)}"
          >{messages.labels.actions.delete.format()}</SvelteButton
        ></span
      >{/each}</span
  >
  <input
    bind:this="{hiddenInput}"
    class="autocomplete__hidden-input"
    tabindex="-1"
    aria-hidden="true"
  />
  <input
    on:input="{onInputOrChange}"
    on:change="{onInputOrChange}"
    on:paste
    name="{fullName}"
    value="{innerUserInput}"
    type="text"
    placeholder="{placeholder}"
    aria-label="{label}"
    disabled="{innerValue.length >= max}"
    required="{innerValue.length < min}"
  />
  <ul>
    {#each truncatedResults as item}
      <li>
        <SvelteButton
          on:click="{() => select(item)}"
          disabled="{innerValue.length >= max || excludedValues.includes(item)}"
          ><slot item="{item}" /></SvelteButton
        >
      </li>
    {/each}
    {#if createCallback && innerUserInput}
      <li>
        <SvelteButton
          on:click="{() => createAndSelect()}"
          disabled="{innerValue.length >= max}"
          >{messages.labels.actions.create.format()}</SvelteButton
        >
      </li>
    {/if}
  </ul>
</div>

<style>
  ul {
    display: none;
  }

  .autocomplete:focus-within ul {
    display: block;
  }

  .autocomplete__hidden-input {
    width: 0;
    height: 0;
    padding: 0;
    margin: 0;
    pointer-events: none;
  }
</style>
