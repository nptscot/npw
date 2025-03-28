<div class="flex-container">
  <div class="top">
    <slot name="top" />
  </div>
  <div class="content">
    <div class="controls">
      <slot name="controls" />
    </div>
    <div class="map">
      <slot name="map" />
    </div>
  </div>
</div>

<style>
  .flex-container {
    display: flex;
    flex-direction: column;
    height: 100vh;
    margin: 0;
  }

  .content {
    display: flex;
    flex: 1;
    overflow: hidden;
  }

  .controls {
    width: 25%;
    min-width: 350px;
  }

  .map {
    width: 75%;
  }
</style>
