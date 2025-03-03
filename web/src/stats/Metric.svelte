<script lang="ts">
  export let label: string;
  export let pct: number;
</script>

<div style="display: flex; flex-direction: column">
  <span>
    {label}:
    <b>{Math.round(pct * 100)}%</b>
  </span>
  <progress value={pct * 100} max="100" />
</div>
