<script lang="ts">
  import type { AppMode } from "$lib/AppMode";
  import type { Game } from "$lib/model/Game";
  import { GameViewModel } from "$lib/viewmodel/GameViewModel";
  import EncounterPage from "../components/EncounterPage.svelte";
  import LibraryPage from "../components/LibraryPage.svelte";
  import MapPage from "../components/MapPage.svelte";
  import Toolbar from "../components/Toolbar.svelte";
  import TradePage from "../components/TradePage.svelte";
  import "../styles/app.scss";
  import { currentTheme } from "../themes/theme";

  let mode: AppMode = "encounter";
  currentTheme.set("Dungeoneer Dusk");

  let game: Game = {
    participants: [
      {
        player: {
          def: {
            name: "Heroic Joe",
            classes: [{ class: "Fighter", level: 5 }],
            stats: {
              str: 16,
              dex: 14,
              con: 15,
              int: 10,
              wis: 12,
              cha: 8,
            },
            ac: 18,
            initiativeBonus: 0,
            portrait: "unknown-player",
            notes:
              "Lawful stupid murder hobo with an angsty backstory who roams the land in search of justice or something.",
          },
          initiative: 15,
          tiebreaker: 123,
          action: true,
          reaction: true,
          bonusAction: true,
          notes: "",
          conditions: [{ name: "surprised" }, { name: "concentrating" }],
        },
      },
      {
        monster: {
          name: "Gobbo McGobface",
          subtype: "Small Humanoid (Goblin)",
          portrait: "goblin",
          cr: 1,
          ac: 14,
          stats: {
            str: 8,
            dex: 15,
            con: 10,
            int: 10,
            wis: 8,
            cha: 8,
          },
          initiativeBonus: 0,
          hitDice: "3d6",
          initiative: 12,
          tiebreaker: -456,
          hp: 8,
          tempHp: 0,
          maxHp: 16,
          action: true,
          reaction: true,
          bonusAction: true,
          legendaryActions: 0,
          notes:
            "Gobbo McGobface is a complex and multidimensional character with hopes, dreams, and a knife. He's green",
          conditions: [
            { name: "bloodied" },
            { name: "dead" },
            { name: "frightened", duration: 42 },
          ],
        },
      },
      {
        monster: {
          name: "Froggo McFrogface",
          subtype: "Medium Humanoid",
          portrait: "bullywug",
          cr: 1,
          ac: 14,
          stats: {
            str: 12,
            dex: 12,
            con: 13,
            int: 7,
            wis: 10,
            cha: 7,
          },
          initiativeBonus: 0,
          hitDice: "2d8+2",
          initiative: 19,
          tiebreaker: -456,
          hp: 1234,
          tempHp: 3,
          maxHp: 16,
          action: true,
          reaction: true,
          bonusAction: true,
          legendaryActions: 0,
          notes: "Froggo McFrogface would rather be eating flies.",
          conditions: [
            { name: "bloodied" },
            { name: "prone" },
            {
              name: "poisoned",
              startTime: 16,
              duration: 60,
            },
            {
              name: "blinded",
              startTime: 6,
              duration: 60,
            },
          ],
        },
      },
      {
        lair: {
          name: "Bullywug Lair",
          notes: "A swampy lair filled with bullywugs.",
          action: true,
        },
      },
    ],
    round: 4,
    turn: 1,
    turnStarted: new Date(),
  };

  let gameViewModel = new GameViewModel(game);
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
