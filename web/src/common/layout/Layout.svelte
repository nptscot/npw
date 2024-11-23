<div class="top">
  <slot name="top" />
</div>
<div class="main-row">
  <div class="left">
    <slot name="left" />
  </div>
  <div class="main">
    <slot name="main" />
  </div>
  <div class="right">
    <slot name="right" />
  </div>
</div>

<style>
  div {
    float: left;
  }

  .top {
    width: 100%;
    height: 6vh;
  }

  .main-row {
    height: 94vh;
    width: 100%;
  }

  .left,
  .right {
    width: 25%;
    height: 100%;
    overflow: scroll;
    padding: 8px;
  }

  .main {
    width: 50%;
    height: 100%;
  }
</style>
