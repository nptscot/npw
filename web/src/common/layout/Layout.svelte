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
    width: 25%; /* padding: 10px; */
    min-width: 350px;

    /* TODO no effect? */
    display: flex;
    flex-direction: column;
    justify-content: space between;
  }

  .map {
    width: 75%;
  }
</style>
