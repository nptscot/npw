<script lang="ts">
  export let label: string;
  export let pct: number;
  export let tooltip: string | undefined = undefined;
</script>

<p title={tooltip}>
  {label}:
  <strong>{Math.round(pct * 100)}%</strong>
  <br />
  <progress value={pct * 100} max="100"></progress>
</p>
