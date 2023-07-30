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

  $: allSelected = computeAllSelected(list, checked);

  /**
   * @param {ItemType[]} list
   * @param {{[ItemType]: boolean}} checked
   * @return {boolean|undefined} true if all selected, false if none selected, undefined if some selected
   */
  function computeAllSelected(list, checked) {
    let allSelected = undefined;
    for (const item of list) {
      if (checked[item]) {
        if (allSelected === true) {
          // continue checking if all are selected
          continue;
        }
        if (allSelected === false) {
          // some are selected
          return undefined;
        }
        if (allSelected === undefined) {
          // we are at the beginning
          allSelected = true;
        }
      } else {
        if (allSelected === true) {
          // some are not selected
          return undefined;
        }
        if (allSelected === false) {
          // continue checking if none are selected
          continue;
        }
        if (allSelected === undefined) {
          // we are at the beginning
          allSelected = false;
        }
      }
    }
    return allSelected;
  }

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
  aria-label="{messages.labels.descriptions.bulkActions.selectAllItems
    .resolveMessage()
    .toString()}"
  checked="{allSelected}"
  indeterminate="{allSelected === undefined}"
/>
<ul aria-live="polite">
  {#each list as item (item)}
    <li>
      <input
        type="checkbox"
        aria-label="{messages.labels.descriptions.bulkActions.selectItem
          .resolveMessage()
          .toString()}"
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
