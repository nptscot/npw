<script lang="ts">
  export let label: string;
  export let showLayer: boolean;
  export let pct: number;
</script>

<!-- svelte-ignore a11y-invalid-attribute -->
<a href="#" on:click|preventDefault={() => (showLayer = true)}>
  {label}
</a>
:
<b>{Math.round(pct * 100)}%</b>
<progress value={pct * 100} max="100" />
