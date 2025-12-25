<script lang="ts">
  import { availableModes, currentTheme, currentThemeMode, themes } from "../../lib/theme";

  let modes = availableModes($currentTheme);

  function themeChanged() {
    modes = availableModes($currentTheme);
    if (!modes.includes($currentThemeMode)) {
      currentThemeMode.set(modes[0]);
    }
  }

  currentTheme.subscribe(() => {
    themeChanged();
  });

  currentThemeMode.subscribe((mode) => {
    if (!modes.includes(mode)) {
      currentThemeMode.set(modes[0]);
    }
  });
</script>

<div>
  <label for="theme"><h2>Theme</h2></label>
  <label for="mode"><h2>Mode</h2></label>

  <select id="theme" bind:value={$currentTheme} on:change={themeChanged}>
    {#each themes as themeName}
      <option value={themeName}>{themeName}</option>
    {/each}
  </select>

  <select id="mode" bind:value={$currentThemeMode}>
    {#each modes as modeName}
      <option value={modeName}>{modeName}</option>
    {/each}
  </select>
</div>

<style lang="scss">
  div {
    display: grid;
    gap: var(--vertical-gap) var(--horizontal-gap);
    grid-template-columns: 1fr 1fr;
  }

  select {
    width: 100%;
  }
</style>
