<script lang="ts">
  import type { AppMode } from "$lib/AppMode";
  import { gameCommands } from "$lib/model/Commands";
  import type { Game } from "$lib/model/Game";
  import { currentTheme } from "$lib/theme";
  import { GameViewModel } from "$lib/viewmodel/GameViewModel";
  import { listen, type Event } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import EncounterPage from "../components/encounter/EncounterPage.svelte";
  import LibraryPage from "../components/LibraryPage.svelte";
  import MapPage from "../components/MapPage.svelte";
  import Toolbar from "../components/Titlebar.svelte";
  import TradePage from "../components/TradePage.svelte";
  import "../styles/app.scss";

  let mode: AppMode = "encounter";
  let game: Game;
  currentTheme.set("dusk");

  onMount(async () => {
    game = await gameCommands.getGame();
  });

  listen("game-updated", (event: Event<Game>) => {
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

<div class="app">
  <Toolbar
    {mode}
    {gameViewModel}
    onModeChange={(newMode) => {
      mode = newMode;
    }}
  />

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

<style lang="scss">
  .app {
    display: grid;
    grid-template-rows: auto 1fr;
    gap: var(--vertical-gap) var(--horizontal-gap);
    height: calc(100vh - 2 * var(--vertical-gap));
    max-height: calc(100vh - 2 * var(--vertical-gap));
    overflow: hidden;
  }
</style>
