<script lang="ts">
  import { type AppMode, appModes, type AppSidebarMode, appSidebarModes } from "$lib/AppMode";
  import type { GameViewModel } from "$lib/viewmodel/GameViewModel";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  // import * as os from "@tauri-apps/plugin-os";
  import AppIcon from "./AppIcon.svelte";
  import EncounterToolbar from "./encounter/EncounterToolbar.svelte";

  const appWindow = getCurrentWebviewWindow();
  const platform: string = "windows"; //os.platform();
  let isMaximized = false;

  export let mode: AppMode = "map";
  export let sidebarMode: AppSidebarMode = undefined;
  export let gameViewModel: GameViewModel;

  export let onModeChange: (mode: AppMode) => void = () => {};
  export let onSidebarModeChange: (mode: AppSidebarMode) => void = () => {};

  const unlistenResized = appWindow.onResized(resized);

  async function resized() {
    isMaximized = await appWindow.isMaximized();
  }

  type DecorationStyle = "generic" | "windows" | "mac";
  let decorationStyle: DecorationStyle = "generic";

  if (platform === "windows") {
    decorationStyle = "windows";
  } else if (platform === "macos") {
    decorationStyle = "mac";
  } else {
    decorationStyle = "generic";
  }
</script>

<nav class="panel">
  {#if decorationStyle !== "mac"}
    <div class="icon" role="none" on:dblclick={appWindow.close}>
      <AppIcon />
    </div>
  {/if}
  <div class="mode-select caption">
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
  <div class="view-tools">
    {#if mode === "encounter"}
      <EncounterToolbar game={gameViewModel} />
    {/if}
  </div>
  <div class="caption sidebar-select">
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
  {#if decorationStyle === "windows"}
    <div class="caption decorations windows">
      <button class="toolbar button" on:click={appWindow.minimize}>&#xE921;</button>
      {#if isMaximized}
        <button class="toolbar button" on:click={appWindow.toggleMaximize}>&#xE923;</button>
      {:else}
        <button class="toolbar button" on:click={appWindow.toggleMaximize}>&#xE922;</button>
      {/if}
      <button class="toolbar button danger" on:click={appWindow.close}>&#xE8BB;</button>
    </div>
  {:else if decorationStyle === "mac"}
    <div class="caption decorations mac">
      <button class="toolbar button red" on:click={appWindow.close}>⬤</button>
      <button class="toolbar button yellow" on:click={appWindow.toggleMaximize}>⬤</button>
      <button class="toolbar button green" on:click={appWindow.minimize}>⬤</button>
    </div>
  {:else if decorationStyle === "generic"}
    <div class="caption decorations generic">
      <button class="toolbar button" on:click={appWindow.minimize}>╶╴</button>
      {#if isMaximized}
        <button class="toolbar button" on:click={appWindow.toggleMaximize}>⮻</button>
      {:else}
        <button class="toolbar button" on:click={appWindow.toggleMaximize}>☐</button>
      {/if}
      <button class="toolbar button danger" on:click={appWindow.close}>✕</button>
    </div>
  {/if}
</nav>

<style lang="scss">
  nav {
    max-height: 2rem;
    overflow: hidden;
    text-wrap: none;
  }

  .panel {
    display: grid;
    grid-template-columns: auto auto 1fr auto auto;
    border: none;
  }

  .toolbar.button {
    margin: 0;
  }

  .decorations {
    display: flex;
    margin: 0;
    padding: 0 !important;

    &.windows {
      grid-column: 5;
      grid-row: 1;

      button {
        font-family: "Segoe Fluent Icons", "Segoe MDL2 Assets";
        font-size: 10px;
        width: 48px;
      }
    }

    &.generic {
      grid-column: 5;
      grid-row: 1;
    }

    &.mac {
      grid-column: 1;
      grid-row: 1;

      button {
        background: none !important;
        &:hover,
        &:active {
          background: none;
        }

        &.red {
          color: var(--intense-red-medium-foreground);
          &:hover {
            color: var(--intense-red-high-foreground);
          }
          &:active {
            color: var(--intense-red-low-foreground);
          }
        }

        &.yellow {
          color: var(--intense-yellow-medium-foreground);
          &:hover {
            color: var(--intense-yellow-high-foreground);
          }
          &:active {
            color: var(--intense-yellow-low-foreground);
          }
        }

        &.green {
          color: var(--intense-green-medium-foreground);
          &:hover {
            color: var(--intense-green-high-foreground);
          }
          &:active {
            color: var(--intense-green-low-foreground);
          }
        }
      }
    }
  }

  .icon {
    grid-column: 1;
    grid-row: 1;
    width: 2rem;
    height: 2rem;
    padding: 1px;
    background-color: var(--chrome);
  }

  .mode-select,
  .sidebar-select {
    display: flex;
    margin: 0;
  }

  .mode-select {
    grid-column: 2;
    grid-row: 1;
    padding: 0 1rem 0 0 !important;
  }

  .view-tools {
    grid-column: 3;
    grid-row: 1;
    padding: 0 1rem;
    background-color: var(--base-color-1-high-background);
  }

  .sidebar-select {
    grid-column: 4;
    grid-row: 1;
    padding: 0 0 0 1rem !important;
  }
</style>
