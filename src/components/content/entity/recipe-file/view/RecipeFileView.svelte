<script>
  import { convertFileSrc } from "@tauri-apps/api/tauri";
  import { RECIPE_FILE_URI_SCHEME } from "../../../../../services/protocol.ts";
  import { recipeFileRepository } from "../../../../../services/repository/recipe-file-repository.ts";
  import { isLoading } from "../../../../../services/util/is-loading.ts";

  export let id;

  /**
   * @type {Readable<Loadable<RecipeFileInterface>>}
   */
  let recipeFile;

  $: recipeFile = recipeFileRepository.createStore(id);
  $: mimeType = $recipeFile?.mime.split("/")[0];
  $: src = convertFileSrc($recipeFile?.path, RECIPE_FILE_URI_SCHEME);
</script>

{#if !isLoading($recipeFile)}
  {#if mimeType === "image"}
    <img src="{src}" alt="{$recipeFile.name}" />
  {:else if mimeType === "video"}
    <video muted>
      <source src="{src}" type="{$recipeFile.mime}" />
    </video>
  {:else if mimeType === "audio"}
    <audio src="{src}"></audio>
  {:else}
    <a href="{src}">{src}</a>
  {/if}
{/if}
