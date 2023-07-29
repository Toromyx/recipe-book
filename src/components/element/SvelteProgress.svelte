<!--
@component
This component implements a labelled progress indicator.
-->

<script>
  import { createId } from "../../services/util/create-id.ts";

  /** @type {string} */
  export let label;

  const id = createId();
  const progressId = `${id}_progress`;
</script>

<label for="{progressId}">{label}</label>
<progress id="{progressId}"></progress>
