<script>
  /**
   * @type {'button'|'submit'|'reset'}
   */
  export let type = "button";
  export let disabled = false;
</script>

<button on:click type="{type}" disabled="{disabled}">
  <slot />
</button>
