<script>
  import Router from "svelte-spa-router";
  import { routes } from "./services/router.js";
</script>

<main>
  <Router routes="{routes}" />
</main>
