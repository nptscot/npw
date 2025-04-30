<script lang="ts">
  // TODO Upstream back to svelte-utils
  // Block the rest of the page while 'loading' is non-empty

  export let loading: string;
  export let progress: number | null = null;
</script>

{#if loading}
  <div class="cover">
    {loading}

    {#if progress != null}
      <div>
        {#if progress == 100}
          Setting up...
          <progress style:width="100%" />
        {:else}
          Downloading...
          <progress value={progress} style:width="100%" />
        {/if}
      </div>
    {/if}
  </div>
{/if}

<style>
  .cover {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    z-index: 999;

    color: white;
    font-size: 32px;
  }
</style>
