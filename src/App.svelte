<script>
  import Router from "svelte-spa-router";
  import { routes } from "./services/router.ts";
</script>

<main>
  <Router routes="{routes}" />
</main>
