<script lang="ts">
  import icon5 from "../../assets/CycleLane.jpg";
  import icon6 from "../../assets/MixedTraffic.jpg";
  import icon2 from "../../assets/OffRoad.jpg";
  import icon1 from "../../assets/Segregated.jpg";
  import icon4 from "../../assets/SharedFootway.jpg";
  import { infraTypeMapping } from "../types";

  export let current: string;

  // TODO URL isn't used right now

  let cases = [
    ["Segregated", icon1, "https://www.cyclestreets.net/location/81274/"],
    ["OffRoad", icon2, "https://www.cyclestreets.net/location/86744/"],
    ["SharedFootway", icon4, "https://www.cyclestreets.net/location/92805/"],
    ["CycleLane", icon5, "https://www.cyclestreets.net/location/81341/"],
    ["MixedTraffic", icon6, ""],
  ];
</script>

<p>
  You can use one infrastructure type for this route, instead of automatically
  picking the most appropriate type. If you do this, you're making the promise
  that this type is appropriate to achieve high Level of Service.
</p>
<p>
  Note if this route overlaps another one, this override won't apply there. You
  have to go edit that other route to override it.
</p>

<div style="display: flex; flex-wrap: wrap; gap: 4px">
  {#each cases as [value, img, _url]}
    {@const [label, color] = infraTypeMapping[value]}
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div
      class="card"
      class:current={current == value}
      on:click={() => (current = value)}
    >
      <img src={img} alt={label} />
      <h5 style:color>{label}</h5>
    </div>
  {/each}
</div>

<style>
  .card {
    flex: 0 0 49%;
    cursor: pointer;
  }

  .current {
    background: black;
  }
</style>
