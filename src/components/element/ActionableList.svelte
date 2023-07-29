<script>
  import { messages } from "../../services/translation/en.ts";
  import SvelteButton from "./SvelteButton.svelte";

  /**
   * @typedef {unknown} ItemType
   */

  /** @type {ItemType[]} */
  export let list;
  /** @type {
   *  {
   *    callback: (items: ItemType[]) => void,
   *    label: string,
   *    labelAll: string,
   *    confirmation?: bool
   *  }[]
   * } */
  export let actions;

  /** @type {HTMLInputElement|undefined} */
  let selectAllInput = undefined;
  let checked = {};

  $: allSelected = list.every((item) => checked[item]);

  function onInputSelectAll() {
    const selectAllChecked = selectAllInput.checked;
    for (const item of list) {
      checked[item] = selectAllChecked;
    }
  }
</script>

<input
  bind:this="{selectAllInput}"
  on:input="{onInputSelectAll}"
  type="checkbox"
  aria-label="{messages.labels.descriptions.bulkActions.selectAllItems.format()}"
  checked="{allSelected}"
/>
<ul aria-live="polite">
  {#each list as item (item)}
    <li>
      <input
        type="checkbox"
        aria-label="{messages.labels.descriptions.bulkActions.selectItem.format()}"
        bind:checked="{checked[item]}"
      /><slot item="{item}" />{#each actions as action}<SvelteButton
          on:click="{() => action.callback([item])}"
          confirmation="{action.confirmation}">{action.label}</SvelteButton
        >{/each}
    </li>
  {/each}
</ul>
{#each actions as action}<SvelteButton
    on:click="{() => action.callback(list.filter((item) => checked[item]))}"
    confirmation="{action.confirmation}">{action.labelAll}</SvelteButton
  >{/each}
