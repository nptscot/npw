<script lang="ts">
  export let label: string;
  export let pct: number;
  // TODO Pico tooltip placement is not smart
  export let tooltip: string | undefined = undefined;
</script>

<div style="display: flex; flex-direction: column" data-tooltip={tooltip}>
  <span>
    {label}:
    <b>{Math.round(pct * 100)}%</b>
  </span>
  <progress value={pct * 100} max="100" />
</div>
