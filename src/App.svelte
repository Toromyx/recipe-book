<!--
@component
This root component contains the router and other components which are only needed once.
-->

<script>
  import Router from "svelte-spa-router";
  import UnitList from "./components/content/UnitList.svelte";
  import { routes } from "./services/router.ts";
</script>

<main>
  <Router routes="{routes}" />
  <UnitList />
</main>
