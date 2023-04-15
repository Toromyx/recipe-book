<!--
@component
This component implements a reactive `<datalist>` for usage in the `list` attribute of `<input>` tags.

Just reference the id `unit-list`. This component can only be used once in the document!
-->

<script>
  import { unitList } from "../../services/store/unit-list.ts";
  import { isLoading } from "../../services/util/loadable.ts";
</script>

<datalist id="unit-list">
  {#if !isLoading($unitList)}
    {#each $unitList as unit}
      <option value="{unit}"></option>
    {/each}
  {/if}
</datalist>
