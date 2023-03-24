<script>
  import { createEventDispatcher, getContext, onDestroy } from "svelte";
  import { messages } from "../../../services/translation/en.ts";
  import { debounce } from "../../../services/util/debounce.ts";
  import { setCustomValidity } from "../../../services/util/validity.ts";
  import SvelteButton from "../SvelteButton.svelte";
  import { FORM } from "./SvelteForm.svelte";

  export let label;
  export let name;
  /** @type {(string) => Promise<unknown[]>} */
  export let callback;
  /** @type {(string) => Promise<void>} */
  export let createCallback = undefined;
  export let value = [];
  export let initialInput = "";
  export let excludedValues = [];
  export let placeholder = label;
  export let min = 0;
  export let max = 1;
  export let maxResults = 10;
  export let debounceWait = 200;

  let userInput = initialInput;

  const formContext = getContext(FORM);
  const dispatch = createEventDispatcher();
  const fullName = formContext?.name ? `${formContext.name}_${name}` : name;
  const getResults = debounce(async () => {
    if (!userInput) {
      return;
    }
    results = await callback(userInput);
  }, debounceWait);
  const createAndSelect = async () => {
    const createValue = await createCallback(userInput);
    if (createValue) {
      select(createValue);
    }
    results = await callback(userInput);
  };

  let setValue = () => {};
  let setChanged = () => {};
  /** @type {unknown[]} */
  let results = [];
  let input;

  if (formContext) {
    setValue = (v) => formContext.setValue(name, v);
    setChanged = () => formContext.setChanged(name);
    formContext.registerReset(name, () => {
      value = [];
      userInput = "";
    });
  }
  getResults();

  $: setValue(value);
  $: filteredResults = results.filter((result) => !value.includes(result));
  $: truncatedResults = filteredResults.slice(0, maxResults);
  $: {
    if (input) {
      setCustomValidity(
        input,
        ...[
          () =>
            value.length < min
              ? messages.validity.autocomplete.min.format({ min })
              : undefined,
          () =>
            value.length > max
              ? messages.validity.autocomplete.max.format({ max })
              : undefined,
        ],
      );
    }
  }

  function onInputOrChange(event) {
    userInput = event.target.value;
    getResults();
  }

  function select(item) {
    value = [...value, item];
    setChanged();
    dispatch("select", value);
  }

  function deselect(item) {
    value = value.filter((valueItem) => valueItem !== item);
    setChanged();
    dispatch("select", value);
  }

  onDestroy(() => {
    formContext.onDestroy(name);
  });
</script>

<div class="autocomplete">
  <span
    >{#each value as item}<span
        ><slot item="{item}" /><SvelteButton on:click="{() => deselect(item)}"
          >{messages.labels.actions.delete.format()}</SvelteButton
        ></span
      >{/each}</span
  >
  <input
    bind:this="{input}"
    on:input="{onInputOrChange}"
    on:change="{onInputOrChange}"
    on:paste
    name="{fullName}"
    value="{userInput}"
    type="text"
    placeholder="{placeholder}"
    aria-label="{label}"
    disabled="{value.length >= max}"
    required="{value.length < min}"
  />
  <ul>
    {#each truncatedResults as item}
      <li>
        <SvelteButton
          on:click="{() => select(item)}"
          disabled="{value.length >= max || excludedValues.includes(item)}"
          ><slot item="{item}" /></SvelteButton
        >
      </li>
    {/each}
    {#if createCallback && userInput}
      <li>
        <SvelteButton
          on:click="{() => createAndSelect()}"
          disabled="{value.length >= max}"
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
</style>
