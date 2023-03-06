<script>
  import { messages } from "../../../../../services/translation/en.ts";
  import FileInput from "../../../../element/form/FileInput.svelte";
  import SvelteInput from "../../../../element/form/SvelteInput.svelte";

  export let name = undefined;
</script>

<SvelteInput
  label="{messages.labels.entityFields.recipeFile.name.format()}"
  name="name"
  value="{name}"
  required="{true}"
/>
<FileInput
  label="{messages.labels.entityFields.recipeFile.path.format()}"
  name="path"
  required="{true}"
/>
