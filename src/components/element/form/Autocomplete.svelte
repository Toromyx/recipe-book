<script>
  import { createEventDispatcher, getContext, tick } from "svelte";
  import { debounce } from "../../../services/util/debounce.js";
  import SvelteButton from "../SvelteButton.svelte";
  import { FORM } from "./SvelteForm.svelte";
  import { messages } from "../../../services/translation/en.js";

  export let label;
  export let name;
  /** @type {(string) => Promise<unknown[]>} */
  export let callback;
  /** @type {(string) => Promise<void>} */
  export let createCallback = undefined;
  export let value = [];
  export let placeholder = label;
  export let min = 0;
  export let max = 1;
  export let maxResults = 10;
  export let debounceWait = 200;

  let userInput = "";

  const formContext = getContext(FORM);
  const dispatch = createEventDispatcher();
  const fullName = formContext?.name ? `${formContext.name}_${name}` : name;
  const getResults = debounce(async () => {
    results = await callback(userInput);
  }, debounceWait);
  const getCreateResults = async () => {
    await createCallback(userInput);
    results = await callback(userInput);
  };

  let setValue = () => {};
  /** @type {unknown[]} */
  let results = [];

  if (formContext) {
    setValue = (v) => formContext.setValue(name, v);
  }

  $: setValue(value);
  $: filteredResults = results.filter((result) => !value.includes(result));
  $: truncatedResults = filteredResults.slice(0, maxResults);

  function onInputOrChange(event) {
    userInput = event.target.value;
    getResults();
  }

  function select(item) {
    value = [...value, item];
    void tick().then(() => {
      dispatch("select", value);
    });
  }

  function deselect(item) {
    value = value.filter((valueItem) => valueItem !== item);
    void tick().then(() => {
      dispatch("select", value);
    });
  }
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
    on:input="{onInputOrChange}"
    on:change="{onInputOrChange}"
    name="{fullName}"
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
          disabled="{value.length >= max}"><slot item="{item}" /></SvelteButton
        >
      </li>
    {/each}
    {#if !results.length && createCallback && userInput}
      <li>
        <SvelteButton
          on:click="{() => getCreateResults()}"
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
