<script>
  import { unitList } from "../../services/store/unit-list.ts";
  import { isLoading } from "../../services/util/is-loading.ts";
</script>

<datalist id="unit-list">
  {#if !isLoading($unitList)}
    {#each $unitList as unit}
      <option value="{unit}"></option>
    {/each}
  {/if}
</datalist>
