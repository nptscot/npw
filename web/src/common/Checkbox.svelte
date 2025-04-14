<script lang="ts">
  import { v4 as uuidv4 } from "uuid";

  export let checked: boolean;
  export let small = false;
  // The slot is the label

  let id = uuidv4();
</script>

<label>
  <input bind:checked type="checkbox" />
  <slot />
</label>
