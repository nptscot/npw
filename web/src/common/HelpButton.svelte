<script lang="ts">
  // This launches a modal when clicked. The user should put the modal contents
  // as a slot beneath this component.
  import icon from "../../assets/help.svg?url";
  import { Modal } from "./index";

  let show = false;
</script>

<button on:click={() => (show = true)}>
  <img src={icon} title="Help" alt="Help" />
</button>

<Modal bind:show>
  <h2>Help</h2>
  <slot />
</Modal>
