<script>
  import RecipeFull from "../content/entity/recipe/RecipeFull.svelte";

  export let params;
</script>

<div>
  <RecipeFull id="{Number(params.id)}" />
</div>
