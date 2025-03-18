<script lang="ts">
  // This launches a modal when clicked. The user should put the modal contents
  // as a slot beneath this component.
  import icon from "../../assets/help.svg?url";
  import { Modal } from "./index";

  let show = false;
</script>

<!-- svelte-ignore a11y-invalid-attribute -->
<a href="#" on:click|preventDefault={() => (show = true)}>
  <img src={icon} alt="Help" />
</a>

<Modal bind:show>
  <h2>Help</h2>
  <slot />
</Modal>

<style>
  img {
    float: right;
    height: 16px;
  }
</style>
