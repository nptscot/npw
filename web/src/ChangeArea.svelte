<script lang="ts">
  export let area: string;

  // jq -r '.features[] | "\"\(.properties.kind)_\(.properties.name)\","' boundaries.geojson
  let areas = [
    "REGION_Aberdeen and North East",
    "REGION_Edinburgh and Lothians",
    "REGION_Glasgow and Strathclyde",
    "REGION_Highlands and Islands",
    "REGION_Scotland South",
    "REGION_Tayside, Central and Fife",
    "LAD_Clackmannanshire",
    "LAD_Dumfries and Galloway",
    "LAD_East Ayrshire",
    "LAD_East Lothian",
    "LAD_East Renfrewshire",
    "LAD_Na h-Eileanan Siar",
    "LAD_Falkirk",
    "LAD_Highland",
    "LAD_Inverclyde",
    "LAD_Midlothian",
    "LAD_Moray",
    "LAD_North Ayrshire",
    "LAD_Orkney Islands",
    "LAD_Scottish Borders",
    "LAD_Shetland Islands",
    "LAD_South Ayrshire",
    "LAD_South Lanarkshire",
    "LAD_Stirling",
    "LAD_Aberdeen City",
    "LAD_Aberdeenshire",
    "LAD_Argyll and Bute",
    "LAD_City of Edinburgh",
    "LAD_Renfrewshire",
    "LAD_West Dunbartonshire",
    "LAD_West Lothian",
    "LAD_Angus",
    "LAD_Dundee City",
    "LAD_East Dunbartonshire",
    "LAD_Fife",
    "LAD_Perth and Kinross",
    "LAD_Glasgow City",
    "LAD_North Lanarkshire",
  ];
  areas.sort();

  function reload() {
    window.location.href = `index.html?boundary=${area}`;
  }
</script>

<label>
  Area:
  <select bind:value={area} on:change={reload}>
    {#each areas as value}
      <option {value}>{value}</option>
    {/each}
  </select>
</label>
