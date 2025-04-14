<script lang="ts">
  export let checked: boolean;
  export let small = false;
  // The slot is the label

  if (small && false) {
  }
</script>

<label>
  <input bind:checked type="checkbox" />
  <slot />
</label>
