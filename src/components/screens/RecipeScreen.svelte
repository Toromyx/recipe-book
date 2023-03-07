<script>
  import { onDestroy } from "svelte";
  import RecipeView from "../content/entity/recipe/view/RecipeView.svelte";

  export let params;

  const wakeLockSentinelPromise = navigator.wakeLock.request();

  onDestroy(() => {
    void wakeLockSentinelPromise.then((wakeLockSentinel) =>
      wakeLockSentinel.release(),
    );
  });
</script>

<div>
  <RecipeView id="{Number(params.id)}" />
</div>
