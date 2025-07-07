<script lang="ts">
  import {
    type AppMode,
    appModes,
    type AppSidebarMode,
    appSidebarModes,
  } from "$lib/AppMode";

  export let mode: AppMode = "map";
  export let sidebarMode: AppSidebarMode = undefined;

  export let onModeChange: (mode: AppMode) => void = () => {};
  export let onSidebarModeChange: (mode: AppSidebarMode) => void = () => {};
</script>

<nav class="panel">
  <div class="caption mode-select">
    {#each appModes as m}
      <input
        type="radio"
        name="mode"
        class="toolbar button"
        id={m}
        value={m}
        bind:group={mode}
        on:change={() => onModeChange(m)}
      />
      <label for={m}>{m.charAt(0).toUpperCase() + m.slice(1)}</label>
    {/each}
  </div>
  <div></div>
  <div class="caption mode-select">
    {#each appSidebarModes as m}
      <input
        type="radio"
        name="sidebarMode"
        class="toolbar button"
        id={m}
        value={m}
        bind:group={sidebarMode}
        on:change={() => onSidebarModeChange(m)}
      />
      <label for={m}>{m!.charAt(0).toUpperCase() + m!.slice(1)}</label>
    {/each}
  </div>
</nav>

<style lang="scss">
  .panel {
    display: grid;
    grid-template-columns: auto 1fr auto;
  }

  .mode-select {
    display: flex;
    margin: 0;
    padding: 0 var(--horizontal-gap) !important;
  }
</style>
