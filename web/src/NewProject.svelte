<script lang="ts">
  import { listFilesInBoundary } from "./common/files";
  import { autosave, boundaryName, mode, setCurrentFile } from "./stores";

  export let subpage: "explore" | "project-list" | "new-project";

  let name = "";
  let fileList = listFilesInBoundary($boundaryName);

  async function newFile() {
    setCurrentFile(name);
    // Immediately save the blank file
    await autosave();
    $mode = { kind: "overview" };
  }
</script>

<header class="ds_page-header">
  <h2 class="ds_page-header__title">New network design</h2>
</header>

<div>
  <button
    type="button"
    class="ds_link"
    on:click={() => (subpage = "project-list")}
  >
    <i class="fa-solid fa-chevron-left"></i>
    Back
  </button>
</div>

<p>Enter a title for the network.</p>

<input
  class="ds_input ds_input--fixed-20"
  placeholder="Name"
  bind:value={name}
/>

<p>
  The NPT provides a set of starting points for the network, or you can start
  from a blank map.
</p>

<p>Or clone from your existing projects</p>

<button class="ds_button" disabled={name.length == 0} on:click={newFile}>
  Start designing
</button>
