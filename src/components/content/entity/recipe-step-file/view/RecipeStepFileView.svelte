<!--
This component display a recipe step file.

It includes functionality to optically recognize characters in the recipe step file.
-->

<script>
  import { convertFileSrc } from "@tauri-apps/api/tauri";
  import { invoke } from "../../../../../services/command/client.ts";
  import { Command } from "../../../../../services/command/command.ts";
  import { recipeStepFileRepository } from "../../../../../services/store/repository/recipe-step-file-repository.ts";
  import { messages } from "../../../../../services/translation/en.ts";
  import { createId } from "../../../../../services/util/create-id.ts";
  import { isLoaded } from "../../../../../services/util/loadable.ts";
  import SvelteButton from "../../../../element/SvelteButton.svelte";

  /**
   * the id of the recipe step file
   */
  export let id;

  const uuid = createId();
  const buttonId = `${uuid}-button`;

  /**
   * @type {Readable<Loadable<RecipeStepFileInterface>>}
   */
  let recipeStepFile;
  let output;
  let iframe;

  $: recipeStepFile = recipeStepFileRepository.createStore(id);
  $: mimeType = $recipeStepFile?.mime.split("/")[0];
  $: src = convertFileSrc($recipeStepFile?.path);
  $: {
    if (iframe) {
      iframe.srcdoc = `<!DOCTYPE html>${output}`;
    }
  }
  $: iframeDisplay = output ? "block" : "none";
</script>

{#if isLoaded($recipeStepFile)}
  {#if mimeType === "image"}
    <img src="{src}" alt="{$recipeStepFile.name}" />
  {:else if mimeType === "video"}
    <video muted controls>
      <source src="{src}" type="{$recipeStepFile.mime}" />
    </video>
  {:else if mimeType === "audio"}
    <audio src="{src}"></audio>
  {:else}
    <a href="{src}">{src}</a>
  {/if}
  <SvelteButton
    id="{buttonId}"
    on:click="{() =>
      void invoke(Command.OCR, { recipeStepFileId: id }).then((result) => {
        output = result;
      })}">{messages.labels.actions.ocr.format()}</SvelteButton
  >
  <output for="{buttonId}">
    <iframe
      bind:this="{iframe}"
      title="hocr"
      style:display="{iframeDisplay}"
      style:resize="both"
    ></iframe>
  </output>
{/if}
