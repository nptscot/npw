<script lang="ts">
  export let list: any[];
  export let idx = 0;

  function prev() {
    if (idx != 0) {
      idx--;
    }
  }

  function next() {
    if (idx != list.length - 1) {
      idx++;
    }
  }
</script>

<div
  style="display: flex; justify-content: space-between; align-items: center;"
>
  <button
    class="outline"
    disabled={idx == 0}
    on:click={prev}
    data-tooltip="Left"
  >
    Previous
  </button>
  {idx + 1} / {list.length}
  <button
    class="outline"
    disabled={idx == list.length - 1}
    on:click={next}
    data-tooltip="Right"
  >
    Next
  </button>
</div>
