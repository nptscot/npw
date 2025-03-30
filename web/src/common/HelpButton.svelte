<script lang="ts">
  // This launches a modal when clicked. The user should put the modal contents
  // as a slot beneath this component.
  import { Modal } from "./index";

  let show = false;
</script>

<button type="button" class="ds_link" on:click={() => (show = true)}>
  <i class="fa-solid fa-circle-info"></i>
</button>

<Modal bind:show>
  <h2>Help</h2>
  <slot />
</Modal>
