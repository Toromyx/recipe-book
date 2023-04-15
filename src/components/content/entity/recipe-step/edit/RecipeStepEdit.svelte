<!--
@component
This component implements form fields to edit a recipe step.
-->

<script>
  import { messages } from "../../../../../services/translation/en.ts";
  import SvelteTextarea from "../../../../element/form/SvelteTextarea.svelte";

  /**
   * the existing recipe step's description
   * @type {?string}
   */
  export let description = undefined;
</script>

<SvelteTextarea
  name="description"
  value="{description}"
  label="{messages.labels.entityFields.recipeStep.description.format()}"
  required="{true}"
/>
