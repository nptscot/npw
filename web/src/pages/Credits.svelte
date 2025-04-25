<script lang="ts">
  import "@scottish-government/design-system/dist/css/design-system.css";
  import "@scottish-government/design-system/dist/scripts/design-system.js";
  import "@fortawesome/fontawesome-free/css/all.min.css";
  import { onMount } from "svelte";

  onMount(async () => {
    // @ts-expect-error This really exists for the SG design system, but TS doesn't know about it
    window.DS.initAll();
  });
</script>

<h1>Network Planning Workspace</h1>

<div>
  <a href="./">
    <i class="fa-solid fa-chevron-left"></i>
    Back to NPW
  </a>
</div>

<p>
  The NPW is designed to enable local authorities to plan a cycle network for
  the area, using segregated infrastructure on key routes and ensuring local
  places are properly connected without severance.
</p>

<p>
  The NPW is an <a href="https://nptscot.github.io/npw/" target="_blank">
    open source project
  </a>
  funded by
  <a href="https://www.transport.gov.scot/" target="_blank">
    Transport Scotland
  </a>
  and designed by
  <a href="https://www.sustrans.org.uk" target="_blank">Sustrans Scotland</a>
  . The team includes:
</p>
<ul>
  <li>
    Dustin Carlino, lead developer and director of <a
      href="https://find-and-update.company-information.service.gov.uk/company/16217327"
      target="_blank"
    >
      A/B Street Ltd
    </a>
  </li>

  <li>
    <a href="https://www.cyclestreets.org/" target="_blank">CycleStreets Ltd</a>
    , UX/workflow design
  </li>

  <li>
    Sustrans Scotland
    <ul>
      <li>
        <a href="https://www.linkedin.com/in/congying-hu/" target="_blank">
          Congying (Cici) Hu
        </a>
        , mobility planner
      </li>
      <li>
        <a
          href="https://www.linkedin.com/in/angus-calder-a69543206"
          target="_blank"
        >
          Angus Calder
        </a>
        , senior mobility planner
      </li>
      <li>
        <a
          href="https://www.linkedin.com/in/matthew-davis-6b3a8124/"
          target="_blank"
        >
          Matthew Davis
        </a>
        , principal mobility planner
      </li>
    </ul>
  </li>

  <li>
    ITS Leeds data science
    <ul>
      <li>
        <a href="https://www.robinlovelace.net/" target="_blank">
          Robin Lovelace
        </a>
      </li>
      <li>
        <a href="https://github.com/wangzhao0217" target="_blank">Zhao Wang</a>
      </li>
      <li>
        <a href="https://github.com/juanfonsecaLS1" target="_blank">
          Juan P. Fonseca-Zamora
        </a>
      </li>
    </ul>
  </li>
</ul>

<p>This tool would not be possible without:</p>
<ul>
  <li>
    <a href="https://www.openstreetmap.org/about" target="_blank">
      OpenStreetMap contributors
    </a>
  </li>
  <li><a href="https://www.maptiler.com/" target="_blank">Maptiler</a></li>
  <li><a href="https://georust.org/" target="_blank">GeoRust</a></li>
  <li>
    <a href="https://fontawesome.com/" target="_blank">Font Awesome icons</a>
  </li>
  <li>And countless other open source libraries</li>
</ul>
