<script>
  import { onDestroy } from "svelte";
  import RecipeFull from "../content/entity/recipe/RecipeFull.svelte";

  export let params;

  const wakeLockSentinelPromise = navigator.wakeLock.request();

  onDestroy(() => {
    void wakeLockSentinelPromise.then((wakeLockSentinel) =>
      wakeLockSentinel.release(),
    );
  });
</script>

<div>
  <RecipeFull id="{Number(params.id)}" />
</div>
