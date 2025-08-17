<script lang="ts">
  import type { AppMode } from "$lib/AppMode";
  import { game_commands } from "$lib/model/Commands";
  import type { Game } from "$lib/model/Game";
  import { GameViewModel } from "$lib/viewmodel/GameViewModel";
  import { onMount } from "svelte";
  import EncounterPage from "../components/EncounterPage.svelte";
  import LibraryPage from "../components/LibraryPage.svelte";
  import MapPage from "../components/MapPage.svelte";
  import Toolbar from "../components/Toolbar.svelte";
  import TradePage from "../components/TradePage.svelte";
  import "../styles/app.scss";
  import { currentTheme } from "../themes/theme";

  let mode: AppMode = "encounter";
  let game: Game;
  currentTheme.set("dusk");

  onMount(async () => {
    game = await game_commands.getGame();
    // console.log(JSON.stringify(game, undefined, 2));
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
