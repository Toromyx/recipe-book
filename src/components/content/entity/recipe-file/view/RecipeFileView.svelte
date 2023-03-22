<script>
  import { convertFileSrc } from "@tauri-apps/api/tauri";
  import { client } from "../../../../../services/command/client.ts";
  import { Command } from "../../../../../services/command/command.ts";
  import { RECIPE_FILE_URI_SCHEME } from "../../../../../services/protocol.ts";
  import { recipeFileRepository } from "../../../../../services/store/repository/recipe-file-repository.ts";
  import { messages } from "../../../../../services/translation/en.ts";
  import { createId } from "../../../../../services/util/create-id.ts";
  import { isLoading } from "../../../../../services/util/is-loading.ts";
  import SvelteButton from "../../../../element/SvelteButton.svelte";

  export let id;

  const uuid = createId();
  const buttonId = `${uuid}-button`;

  /**
   * @type {Readable<Loadable<RecipeFileInterface>>}
   */
  let recipeFile;
  let output;
  let iframe;

  $: recipeFile = recipeFileRepository.createStore(id);
  $: mimeType = $recipeFile?.mime.split("/")[0];
  $: src = convertFileSrc($recipeFile?.path, RECIPE_FILE_URI_SCHEME);
  $: {
    if (iframe) {
      iframe.srcdoc = `<!DOCTYPE html>${output}`;
    }
  }
  $: iframeDisplay = output ? "block" : "none";
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
  <SvelteButton
    id="{buttonId}"
    on:click="{() =>
      void client.invoke(Command.OCR, { recipeFileId: id }).then((result) => {
        output = result;
      })}">{messages.labels.actions.ocr.format()}</SvelteButton
  >
  <output for="{buttonId}">
    <iframe
      bind:this="{iframe}"
      title="hocr"
      style:display="{iframeDisplay}"
      style:resize="both"></iframe>
  </output>
{/if}
