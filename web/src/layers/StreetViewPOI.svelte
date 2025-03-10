<script lang="ts">
  import { Control } from "svelte-maplibre";
  import { currentPOI } from "./stores";
</script>

{#if $currentPOI}
  <Control position="bottom-left">
    <div>
      <a
        href={`http://maps.google.com/maps?q=&layer=c&cbll=${$currentPOI.pt[1]},${$currentPOI.pt[0]}&cbp=11,0,0,0,0`}
        target="_blank"
      >
        Streetview for {$currentPOI.pt}
      </a>
    </div>
  </Control>
{/if}

<style>
  div {
    background: white;
  }
</style>
