<!--
@component
This screen component displays an recipe.

The recipe is specified through the route parameter `id`.

It acquires a wake lock on initialization to avoid the screen going dark while displaying the recipe.
-->

<script>
  import { onDestroy } from "svelte";
  import RecipeView from "../content/entity/recipe/view/RecipeView.svelte";

  export let params;

  const wakeLockSentinelPromise = navigator.wakeLock?.request("screen");

  onDestroy(() => {
    void wakeLockSentinelPromise?.then((wakeLockSentinel) =>
      wakeLockSentinel.release(),
    );
  });
</script>

<div>
  <RecipeView id="{Number(params.id)}" />
</div>
