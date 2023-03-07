<script>
  import { messages } from "../../../../../services/translation/en.ts";
  import SvelteTextarea from "../../../../element/form/SvelteTextarea.svelte";

  export let description = undefined;
</script>

<SvelteTextarea
  name="description"
  value="{description}"
  label="{messages.labels.entityFields.recipeStep.description.format()}"
  required="{true}"
/>
