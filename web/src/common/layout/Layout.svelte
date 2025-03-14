<div class="flex-container">
  <div class="top">
    <slot name="top" />
  </div>
  <div class="content">
    <div class="left">
      <slot name="left" />
    </div>
    <div class="map">
      <slot name="map" />
    </div>
  </div>
</div>

<style>
  /* Legacy note: calling this "container" will clash with a pico built-in */
  .flex-container {
    display: flex;
    flex-direction: column;
    height: 100vh;
    margin: 0;
  }

  .top {
    width: 100%;
    padding: 4px;
  }

  .content {
    display: flex;
    flex: 1;
    overflow: hidden;
  }

  .left {
    width: 25%;
    overflow-y: auto;
    padding: 4px;
  }

  .map {
    width: 75%;
  }
</style>
