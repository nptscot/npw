<script lang="ts">
  import { v4 as uuidv4 } from "uuid";

  export let value: string;
  // [radioValue, label]
  export let options: [string, string][];
  export let legend: string;
  export let small = false;
</script>

<fieldset>
  <legend>{legend}</legend>
  <div class="ds_field-group">
    {#each options as [radioValue, label]}
      {@const id = uuidv4()}
      <div class:ds_radio={true} class:ds_radio--small={small}>
        <input
          class="ds_radio__input"
          {id}
          name={id}
          type="radio"
          value={radioValue}
          bind:group={value}
          on:change
        />
        <label class="ds_radio__label" for={id}>{label}</label>
      </div>
    {/each}
  </div>
</fieldset>
