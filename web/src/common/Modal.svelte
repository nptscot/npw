<script lang="ts">
  export let show = false;

  let modalDialog: HTMLDialogElement | undefined;

  $: {
    if (modalDialog) {
      if (show) {
        modalDialog.showModal();
      } else {
        modalDialog.close();
      }
    }
  }

  function onClick(e: MouseEvent) {
    // only dismiss the modal when clicking outside of the inner dialog content, on the dialog itself.
    if (e.target == modalDialog) {
      show = false;
    }
  }

  function onKeyDown(e: KeyboardEvent) {
    if (e.key == "Escape" || e.key == "Enter") {
      e.stopPropagation();
      show = false;
    }
  }
</script>

<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<dialog
  bind:this={modalDialog}
  on:click|stopPropagation={onClick}
  on:keydown={onKeyDown}
>
  <slot />
</dialog>

<style>
  dialog {
    max-width: 80%;
    max-height: 80%;
  }

  dialog::backdrop {
    backdrop-filter: blur(2px);
  }
</style>
