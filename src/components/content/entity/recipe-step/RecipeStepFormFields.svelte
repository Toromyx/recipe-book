<script>
  import { messages } from "../../../../services/translation/en.ts";
  import SvelteInput from "../../../element/form/SvelteInput.svelte";
  import SvelteTextarea from "../../../element/form/SvelteTextarea.svelte";

  export let description = undefined;
</script>

<SvelteTextarea
  name="description"
  value="{description}"
  label="{messages.labels.entityFields.recipeStep.description.format()}"
  required="{true}"
/>
<SvelteInput
  type="file"
  name="image"
  label="{messages.labels.entityFields.recipeStep.image.format()}"
  accept="image/*"
/>
