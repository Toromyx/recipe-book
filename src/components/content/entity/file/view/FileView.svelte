<!--
This component displays a file.

It includes functionality to optically recognize characters in the file.
-->

<script>
  import { convertFileSrc } from "@tauri-apps/api/tauri";
  import { invoke } from "../../../../../services/command/client.ts";
  import { Command } from "../../../../../services/command/command.ts";
  import { fileRepository } from "../../../../../services/store/repository/file-repository.ts";
  import { messages } from "../../../../../services/translation/en.ts";
  import { createId } from "../../../../../services/util/create-id.ts";
  import { isLoaded } from "../../../../../services/util/loadable.ts";
  import SvelteButton from "../../../../element/SvelteButton.svelte";

  /**
   * the id of the file
   */
  export let id;

  const uuid = createId();
  const buttonId = `${uuid}-button`;

  /**
   * @type {Readable<Loadable<FileInterface>>}
   */
  let file;
  let output;
  let iframe;

  $: file = fileRepository.createStore(id);
  $: mimeType = $file?.mime.split("/")[0];
  $: src = convertFileSrc($file?.path);
  $: {
    if (iframe) {
      iframe.srcdoc = `<!DOCTYPE html>${output}`;
    }
  }
  $: iframeDisplay = output ? "block" : "none";
</script>

{#if isLoaded($file)}
  {#if mimeType === "image"}
    <img src="{src}" alt="{$file.name}" />
  {:else if mimeType === "video"}
    <video muted controls>
      <source src="{src}" type="{$file.mime}" />
    </video>
  {:else if mimeType === "audio"}
    <audio src="{src}"></audio>
  {:else}
    <a href="{src}">{src}</a>
  {/if}
  <SvelteButton
    id="{buttonId}"
    on:click="{() =>
      void invoke(Command.OCR, { fileId: id }).then((result) => {
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
