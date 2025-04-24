<script lang="ts">
  import { stageColors } from "../colors";
  import { currentStage } from "../stores";

  export let label: string;
  export let pct: number;
  export let tooltip: string | undefined = undefined;

  $: color = stageColors[$currentStage];
</script>

<p title={tooltip}>
  {label}:
  <strong>{Math.round(pct * 100)}%</strong>
  <br />
  <progress value={pct * 100} max="100" style="--color: {color}"></progress>
</p>

<style>
  strong {
    float: right;
  }

  progress {
    width: 100%;
  }

  p {
    margin-bottom: 12px;
    line-height: 20px;
  }
</style>
