<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { v4 as uuidv4 } from "uuid";

  export let checked: boolean;
  export let small = false;
  // The slot is the label

  let id = uuidv4();
  let dispatch = createEventDispatcher();
</script>

<div class:ds_checkbox={true} class:ds_checkbox--small={small}>
  <input
    bind:checked
    on:change={() => dispatch("change", checked)}
    class="ds_checkbox__input"
    {id}
    name={id}
    type="checkbox"
  />
  <label class="ds_checkbox__label" for={id}>
    <slot />
  </label>
</div>
