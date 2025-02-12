<script lang="ts">
  export let label: string;
  export let pct: number;
</script>

{label}:
<b>{Math.round(pct * 100)}%</b>
<progress value={pct * 100} max="100" />
