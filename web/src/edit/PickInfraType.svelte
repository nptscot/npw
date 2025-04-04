<script lang="ts">
  import cycleLaneIcon from "../../assets/CycleLane.jpg";
  import mixedTrafficIcon from "../../assets/MixedTraffic.jpg";
  import offRoadIcon from "../../assets/OffRoad.jpg";
  import segregatedIcon from "../../assets/Segregated.jpg";
  import segregatedWithSpeedVolumeIcon from "../../assets/SegregatedWithSpeedVolume.jpg";
  import sharedFootwayIcon from "../../assets/SharedFootway.jpg";
  import { infraTypeMapping } from "../types";

  export let onFinish: (infraType: string) => void;

  // TODO URL isn't used right now

  let cases = [
    [
      "Segregated",
      segregatedIcon,
      "https://www.cyclestreets.net/location/81274/",
    ],
    [
      "SegregatedWithSpeedVolume",
      segregatedWithSpeedVolumeIcon,
      "https://www.cyclestreets.net/location/92772/",
    ],
    ["OffRoad", offRoadIcon, "https://www.cyclestreets.net/location/86744/"],
    [
      "SharedFootway",
      sharedFootwayIcon,
      "https://www.cyclestreets.net/location/92805/",
    ],
    [
      "CycleLane",
      cycleLaneIcon,
      "https://www.cyclestreets.net/location/81341/",
    ],
    ["MixedTraffic", mixedTrafficIcon, ""],
  ];
</script>

<p>
  You can use one infrastructure type for this route, instead of automatically
  picking the most appropriate type. If you do this, you're making the promise
  that this type is appropriate to achieve high Level of Service.
</p>
<p>
  Note if this route overlaps another one, this override won't apply there. You
  would need to edit that other route to override it.
</p>

<div style="display: flex; flex-wrap: wrap; gap: 10px">
  {#each cases as [value, img, _url]}
    {@const [label, color] = infraTypeMapping[value]}
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div class="card" on:click={() => onFinish(value)}>
      <img src={img} alt={label} style:height="150px" />
      <h5 style:color>{label}</h5>
    </div>
  {/each}
</div>

<style>
  .card {
    flex: 0 0 30%;
    cursor: pointer;
  }

  .card:hover {
    background: #d3d3d3;
  }
</style>
