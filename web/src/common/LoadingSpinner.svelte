<script lang="ts">
  import { loadingSpinners } from "../stores";
</script>

{#if $loadingSpinners > 0}
  <div>
    <span class="loader" />
  </div>
{/if}

<style>
  div {
    position: absolute;
    top: 80px;
    left: 10px;
    background-color: white;
    padding: 4px;
  }

  /* Thanks to https://cssloaders.github.io/ */
  .loader {
    width: 24px;
    height: 24px;
    border: 5px solid #fff;
    border-bottom-color: #ff3d00;
    border-radius: 50%;
    display: inline-block;
    box-sizing: border-box;
    animation: rotation 1s linear infinite;
  }

  @keyframes rotation {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }
</style>
