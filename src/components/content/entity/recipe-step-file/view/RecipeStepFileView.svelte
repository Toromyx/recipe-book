<!--
This component display a recipe step file.
-->

<script>
  import { recipeStepFileRepository } from "../../../../../services/store/repository/recipe-step-file-repository.ts";
  import { isLoaded } from "../../../../../services/util/loadable.ts";
  import FileView from "../../file/view/FileView.svelte";

  /**
   * the id of the recipe step file
   */
  export let id;

  /**
   * @type {Readable<Loadable<RecipeStepFileInterface>>}
   */
  let recipeStepFile;

  $: recipeStepFile = recipeStepFileRepository.createStore(id);
</script>

{#if isLoaded($recipeStepFile)}
  <FileView id="{$recipeStepFile.fileId}" />
{/if}
