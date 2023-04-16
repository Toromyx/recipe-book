<!--
@component
This component implements a reactive `<datalist>` for usage in the `list` attribute of `<input>` tags.

Just reference the exported id. This component can only be used once in the document!
-->
<script context="module">
  export const UNIT_LIST_ID = "unit-list";
</script>

<script>
  import { unitList } from "../../services/store/unit-list.ts";
  import { isLoading } from "../../services/util/loadable.ts";
</script>

<datalist id="{UNIT_LIST_ID}">
  {#if !isLoading($unitList)}
    {#each $unitList as unit}
      <option value="{unit}"></option>
    {/each}
  {/if}
</datalist>
