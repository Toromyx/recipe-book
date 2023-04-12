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
  export let userInput = "";
  export let excludedValues = [];
  export let placeholder = label;
  export let min = 0;
  export let max = 1;
  export let maxResults = 10;
  export let debounceWait = 200;

  const formContext = getContext(FORM);
  const dispatch = createEventDispatcher();
  const fullName = formContext?.name ? `${formContext.name}_${name}` : name;
  const getResults = debounce(async () => {
    if (!innerUserInput) {
      return;
    }
    results = await callback(innerUserInput);
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
  let input;

  if (formContext) {
    setValue = (v) => formContext.setValue(name, v);
    setChanged = () => formContext.setChanged(name);
    formContext.registerReset(name, () => {
      innerValue = [];
      dispatch("select", innerValue);
      innerUserInput = "";
      dispatch("input", innerUserInput);
    });
  }
  getResults();

  $: innerValue = value;
  $: setValue(innerValue);
  $: innerUserInput = userInput;
  $: filteredResults = results.filter((result) => !innerValue.includes(result));
  $: truncatedResults = filteredResults.slice(0, maxResults);
  $: {
    if (input) {
      setCustomValidity(
        input,
        ...[
          () =>
            innerValue.length < min
              ? messages.validity.autocomplete.min.format({ min })
              : undefined,
          () =>
            innerValue.length > max
              ? messages.validity.autocomplete.max.format({ max })
              : undefined,
        ],
      );
    }
  }

  function onInputOrChange(event) {
    innerUserInput = event.target.value;
    getResults();
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
    bind:this="{input}"
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
</style>
