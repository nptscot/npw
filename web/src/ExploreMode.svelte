<script lang="ts">
  import { SplitComponent } from "./common/layout";
  import { boundaryName, mode } from "./stores";
</script>

<SplitComponent>
  <div slot="controls" class="main-controls">
    <header class="ds_page-header">
      <h2 class="ds_page-header__title">Explore {$boundaryName}</h2>
    </header>

    <div>
      <button
        type="button"
        class="ds_link"
        on:click={() => (window.location.href = "./html")}
      >
        <i class="fa-solid fa-chevron-left"></i>
        Work in a different area
      </button>
    </div>

    <p>Explore the map and layers, or begin a new network project.</p>
    <p>
      You should familiarise yourself with current infrastructure, via the
      layers on the right, before starting your new cycle network.
    </p>

    <div>
      <button class="ds_button" on:click={() => ($mode = { kind: "overview" })}>
        Design my network
      </button>
    </div>
  </div>
</SplitComponent>

<style>
  .main-controls {
    overflow-y: auto;
    padding: 20px;
  }
</style>
