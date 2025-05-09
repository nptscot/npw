<script lang="ts">
  import logo from "../assets/npt_logo.png?url";
  import { tierLabels } from "./colors";
  import { canStopDrawing } from "./edit/stores";
  import TopBarStats from "./stats/TopBarStats.svelte";
  import { changeStage, currentStage, exitCurrentStage, mode } from "./stores";

  let stages = { ...tierLabels, assessment: "Assess" };

  function gotoOverview() {
    if (canStopDrawing()) {
      exitCurrentStage();
      $mode = { kind: "overview" };
    }
  }

  function switchStage(stage: string) {
    if (canStopDrawing()) {
      changeStage(stage);
    }
  }
</script>

<header>
  <div class="site-navigation">
    <nav class="ds_site-navigation">
      <ul class="ds_site-navigation__list">
        <li class="ds_site-navigation__item">
          <a class="ds_site-navigation__link" href="./">
            <img id="logo" src={logo} alt="NPT logo" />
          </a>
        </li>

        {#if $mode.kind != "setup"}
          <li class="ds_site-navigation__item">
            <!-- svelte-ignore a11y-invalid-attribute -->
            <a
              class="ds_site-navigation__link"
              class:ds_current={$mode.kind == "overview"}
              href="#"
              on:click|preventDefault={gotoOverview}
            >
              Overview <i class="fa-solid fa-chevron-right"></i>
            </a>
          </li>

          {#each Object.entries(stages) as [stage, label]}
            <li class="ds_site-navigation__item">
              <!-- svelte-ignore a11y-invalid-attribute -->
              <a
                class="ds_site-navigation__link {stage}"
                class:ds_current={$mode.kind != "overview" &&
                  $currentStage == stage}
                href="#"
                on:click|preventDefault={() => switchStage(stage)}
              >
                {label}
                {#if stage != "assessment"}
                  <i class="fa-solid fa-chevron-right"></i>
                {/if}
              </a>
            </li>
          {/each}
        {/if}
      </ul>
    </nav>
  </div>

  {#if $mode.kind != "setup"}
    <TopBarStats />
  {/if}
</header>

<style>
  :root {
    --primary-tier-colour: #970f52;
    --secondary-tier-colour: #ff978c;
    --localaccess-tier-colour: #feae01;
    --longdistance-tier-colour: #fef157;
  }

  header {
    display: flex;
    padding: 0 20px;
    box-shadow: 0px 10px 10px 0px #eee;
  }
  header #logo {
    height: 30px;
    margin-right: 10px;
    vertical-align: middle;
  }

  .ds_site-navigation .ds_site-navigation__link {
    padding-left: 8px;
    padding-right: 8px;
  }
  .ds_site-navigation .ds_current {
    font-weight: bold;
  }

  .ds_site-navigation .ds_site-navigation__link.Primary:not(:focus):after {
    border-bottom-color: var(--primary-tier-colour);
  }
  .ds_site-navigation .ds_site-navigation__link.Secondary:not(:focus):after {
    border-bottom-color: var(--secondary-tier-colour);
  }
  .ds_site-navigation .ds_site-navigation__link.LocalAccess:not(:focus):after {
    border-bottom-color: var(--localaccess-tier-colour);
  }
  .ds_site-navigation .ds_site-navigation__link.LongDistance:not(:focus):after {
    border-bottom-color: var(--longdistance-tier-colour);
  }

  /* Apply tier accent colour to hover/selected states in menu */
  .ds_site-navigation .ds_site-navigation__link.Primary:hover,
  .ds_site-navigation .ds_site-navigation__link.Primary.ds_current {
    background-color: rgb(from var(--primary-tier-colour) r g b / 0.15);
  }
  .ds_site-navigation .ds_site-navigation__link.Secondary:hover,
  .ds_site-navigation .ds_site-navigation__link.Secondary.ds_current {
    background-color: rgb(from var(--secondary-tier-colour) r g b / 0.15);
  }
  .ds_site-navigation .ds_site-navigation__link.LocalAccess:hover,
  .ds_site-navigation .ds_site-navigation__link.LocalAccess.ds_current {
    background-color: rgb(from var(--localaccess-tier-colour) r g b / 0.15);
  }
  .ds_site-navigation .ds_site-navigation__link.LongDistance:hover,
  .ds_site-navigation .ds_site-navigation__link.LongDistance.ds_current {
    background-color: rgb(from var(--longdistance-tier-colour) r g b / 0.15);
  }
</style>
