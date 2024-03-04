<!--
@component
This component implements form fields for editing a recipe step file.
-->

<script>
  import { messages } from "../../../../../services/translation/en.ts";
  import FileInput from "../../../../element/form/FileInput.svelte";
  import SvelteInput from "../../../../element/form/SvelteInput.svelte";

  /**
   * the existing recipe step file's name
   * @type {?string}
   */
  export let name = undefined;
</script>

<SvelteInput
  label="{messages.labels.entityFields.recipeStepFile.name.format()}"
  name="name"
  value="{name}"
  required="{true}"
/>
<FileInput
  label="{messages.labels.entityFields.recipeStepFile.path.format()}"
  name="path"
  required="{true}"
/>
