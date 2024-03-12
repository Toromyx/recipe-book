<!--
@component
This component implements form fields for editing a file.
-->

<script>
  import { messages } from "../../../../../services/translation/en.ts";
  import FileInput from "../../../../element/form/FileInput.svelte";
  import SvelteInput from "../../../../element/form/SvelteInput.svelte";

  /**
   * the existing file's name
   * @type {?string}
   */
  export let name = undefined;
</script>

<SvelteInput
  label="{messages.labels.entityFields.file.name.format()}"
  name="name"
  value="{name}"
  required="{true}"
/>
<FileInput
  label="{messages.labels.entityFields.file.path.format()}"
  name="path"
  required="{true}"
/>
