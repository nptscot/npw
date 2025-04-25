<script lang="ts">
  import { downloadGeneratedFile } from "svelte-utils";
  import { measureLocalStorageSizes } from "../common/files";

  let items: [string, number][] = measureLocalStorageSizes();
  $: sum = items.reduce((sum, pair) => sum + pair[1], 0);

  function download(filename: string) {
    // Assume most items are GeoJSON
    downloadGeneratedFile(
      `${filename}.geojson`,
      window.localStorage.getItem(filename)!,
    );
  }

  function remove(filename: string) {
    if (window.confirm(`Really delete ${filename}?`)) {
      window.localStorage.removeItem(filename);
      items = measureLocalStorageSizes();
    }
  }

  // Adapted from https://gist.github.com/zentala/1e6f72438796d74531803cc3833c039c
  function formatBytes(bytes: number): string {
    if (bytes == 0) {
      return "0 bytes";
    }
    let k = 1024;
    let sizes = ["bytes", "KB", "MB"];
    let i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + " " + sizes[i];
  }
</script>

<main class="container">
  <h1>Clean local storage</h1>

  <a href="./">Back to NPW</a>

  <p>
    All files are stored in your browser's local storage. There's a 5MB size
    limit, and you've been redirected to this page because the last action you
    took exceeds this limit. You must delete some other files first to continue. <b
    >
      Before you delete a file, you should download a copy.
    </b>
  </p>

  <p>{formatBytes(sum)} / 5 MB is used right now</p>
  <progress
    value={(100 * sum) / (1024 * 1024 * 5)}
    max="100"
    style="width: 100%"
  />

  <table>
    <table>
      <thead>
        <tr>
          <th>Filename</th>
          <th>Size</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        {#each items as [filename, size]}
          <tr>
            <td>{filename}</td>
            <td>{formatBytes(size)}</td>
            <td>
              <button on:click={() => download(filename)}>Download</button>
              <button on:click={() => remove(filename)}>Delete</button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </table>
</main>
