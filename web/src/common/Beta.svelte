<script lang="ts">
  import { Modal } from "./";

  let show = showOnce();

  function showOnce(): boolean {
    let key = "npw-beta-popup";
    if (window.localStorage.getItem(key) != null) {
      return false;
    }
    window.localStorage.setItem(key, "seen");
    return true;
  }

  // The style is based on
  // https://designsystem.gov.scot/components/phase-banner and
  // https://github.com/nptscot/nptscot.github.io/pull/465
</script>

<button type="button" class="banner" on:click={() => (show = true)}>
  beta
</button>

<Modal bind:show>
  <h1>Network Planning Workspace</h1>
  <p>
    This tool utilises various datasets, some of which have <a
      href="manual.html#known-limitations"
      target="_blank"
    >
      limitations
    </a>
    .
  </p>
  <p>
    Demand data: Commute layer results are based on 2011 Census data, and other
    trip purpose results are based on pre-pandemic data. These model-based
    estimates may not reflect current patterns which have changed since the
    pandemic. Updated datasets will be included in future versions.
  </p>
  <p>
    Use the "Report problem" button in the bottom-left corner of the map if you
    find problems, or email the maintainer at <a
      href="mailto:dabreegster@gmail.com"
    >
      dabreegster@gmail.com
    </a>
    .
  </p>
  <button type="button" class="ds_button" on:click={() => (show = false)}>
    Continue
  </button>
</Modal>

<style>
  .banner {
    display: inline-block;
    margin-top: 14px;
    height: 25px;
    line-height: 25px;
    color: #00437d;
    background-color: #d9eeff;
    border: 1px solid rgba(0, 101, 189, 0.8);
    font-weight: normal;
  }
</style>
