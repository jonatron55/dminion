<!--
  Copyright (c) 2025 Jonathon B. Cobb
  Licensed under the MIT License
-->

<script lang="ts">
  import type { AppMode, AppSidebarMode } from "$lib/AppMode";
  import { gameCommands } from "$lib/model/Commands";
  import type { Game } from "$lib/model/Game";
  import { currentTheme, currentThemeMode } from "$lib/theme";
  import { GameViewModel } from "$lib/viewmodel/GameViewModel";
  import { listen, type Event } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import EncounterPage from "../components/encounter/EncounterPage.svelte";
  import LibraryPage from "../components/LibraryPage.svelte";
  import MapPage from "../components/MapPage.svelte";
  import SidebarPanel from "../components/sidebar/SidebarPanel.svelte";
  import Toolbar from "../components/Titlebar.svelte";
  import TradePage from "../components/TradePage.svelte";
  import "../styles/app.scss";

  let mode: AppMode = "encounter";
  let sidebarMode: AppSidebarMode = undefined;
  let game: Game;

  if (window.matchMedia("(prefers-contrast: more)").matches) {
    currentTheme.set("High Contrast");
    if (window.matchMedia("(prefers-color-scheme: dark)").matches) {
      currentThemeMode.set("Dark");
    } else {
      currentThemeMode.set("Light");
    }
  } else {
    currentTheme.set("Dungeoneer");
    if (window.matchMedia("(prefers-color-scheme: dark)").matches) {
      currentThemeMode.set("Dusk");
    } else {
      currentThemeMode.set("Noon");
    }
  }

  onMount(async () => {
    game = await gameCommands.getGame();
  });

  listen("game-updated", (event: Event<Game>) => {
    console.trace(`game-updated ${JSON.stringify(event.payload, null, 2)}`);
    game = event.payload;
  });

  $: game = {
    participants: {},
    order: [],
    round: 0,
    turn: 0,
    gameStarted: new Date(),
    turnStarted: new Date(),
  };

  $: gameViewModel = new GameViewModel(game);
</script>

<div class="app" class:with-sidebar={sidebarMode !== undefined}>
  <div class="titlebar">
    <Toolbar
      {mode}
      {sidebarMode}
      {gameViewModel}
      onModeChange={(newMode) => {
        mode = newMode;
      }}
      onSidebarModeChange={(newMode) => {
        sidebarMode = newMode;
      }}
    />
  </div>

  <div class="main">
    {#if mode === "map"}
      <MapPage />
    {:else if mode === "encounter"}
      <EncounterPage game={gameViewModel} />
    {:else if mode === "trade"}
      <TradePage />
    {:else if mode === "library"}
      <LibraryPage />
    {/if}
  </div>

  {#if sidebarMode !== undefined}
    <div class="sidebar">
      <SidebarPanel mode={sidebarMode} />
    </div>
  {/if}
</div>

<style lang="scss">
  .app {
    display: grid;
    grid-template-rows: auto 1fr;
    grid-template-columns: 1fr;
    grid-template-areas:
      "titlebar"
      "main";
    gap: var(--vertical-gap) var(--horizontal-gap);
    height: calc(100vh - 2 * var(--vertical-gap));
    max-height: calc(100vh - 2 * var(--vertical-gap));
    overflow: hidden;
  }

  .app.with-sidebar {
    grid-template-columns: 1fr var(--sidebar-width);
    grid-template-areas:
      "titlebar titlebar"
      "main sidebar";
  }

  .titlebar {
    grid-area: titlebar;
    overflow: hidden;
  }

  .main {
    grid-area: main;
    overflow: hidden;
  }

  .sidebar {
    grid-area: sidebar;
    overflow: hidden;
  }
</style>
