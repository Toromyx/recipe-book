<script>
  import Router from "svelte-spa-router";
  import UnitList from "./components/content/UnitList.svelte";
  import { routes } from "./services/router.ts";
</script>

<main>
  <Router routes="{routes}" />
  <UnitList />
</main>
