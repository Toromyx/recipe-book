<!--
This component display a recipe file.
-->

<script>
  import { recipeFileRepository } from "../../../../../services/store/repository/recipe-file-repository.ts";
  import { isLoaded } from "../../../../../services/util/loadable.ts";
  import FileView from "../../file/view/FileView.svelte";

  /**
   * the id of the recipe file
   */
  export let id;

  /**
   * @type {Readable<Loadable<RecipeFileInterface>>}
   */
  let recipeFile;

  $: recipeFile = recipeFileRepository.createStore(id);
</script>

{#if isLoaded($recipeFile)}
  <FileView id="{$recipeFile.fileId}" />
{/if}
