<script lang="ts">
  import { currentPOI } from "./stores";

  // Per https://developers.google.com/maps/documentation/embed/quickstart,
  // make an API key and enable the Maps Embed API. This is apparently a
  // free API.
  let apiKey = "AIzaSyA5Na6sqIN4ctBGnA9nGJOiw8ImFcic2SI";

  let show = false;
</script>

<div>
  {#if show && $currentPOI}
    <iframe
      width="600"
      height="200"
      style:border="0"
      loading="lazy"
      allowfullscreen
      title="Google Streetview"
      src={`https://www.google.com/maps/embed/v1/streetview?key=${apiKey}&location=${$currentPOI.pt[1]},${$currentPOI.pt[0]}`}
    />
    <br />
  {/if}
  <button on:click={() => (show = !show)}>
    <i class="fa-solid fa-street-view"></i>
    StreetView
  </button>
</div>

<style>
  div {
    position: absolute;
    bottom: 100px;
    left: 10px;
  }
</style>
