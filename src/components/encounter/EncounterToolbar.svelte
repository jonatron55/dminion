<script lang="ts">
  import { gameCommands } from "$lib/model/Commands";
  import type { GameViewModel } from "$lib/viewmodel/GameViewModel";

  export let game: GameViewModel;

  $: timeStr = (() => {
    const minutes = Math.floor(game.time / 60);
    const seconds = game.time % 60;
    return `${minutes}:${seconds < 10 ? "0" : ""}${seconds}`;
  })();

  function handleKeydown(event: KeyboardEvent) {
    const modifier = event.ctrlKey || event.metaKey; // Ctrl on Windows/Linux, Cmd on Mac

    if (modifier && !event.shiftKey && event.key === "z") {
      event.preventDefault();
      gameCommands.undo();
    } else if (modifier && (event.key === "y" || (event.shiftKey && event.key === "Z"))) {
      event.preventDefault();
      gameCommands.redo();
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="encounter-toolbar">
  <span class="actions">
    <button class="toolbar" on:click={gameCommands.undo}>↺ Undo</button>
    <button class="toolbar" on:click={gameCommands.redo}>↻ Redo</button>
    <button class="toolbar" on:click={gameCommands.nextTurn}>⏩ Next</button>
  </span>
  <span data-tauri-drag-region class="titlebar details">
    <span data-tauri-drag-region class="round">{game.round}</span>
    <span data-tauri-drag-region class="time">{timeStr}</span>
  </span>
  <span class="participants">
    <button class="toolbar">+ Player</button>
    <button class="toolbar">+ Monster</button>
  </span>
</div>

<style lang="scss">
  .encounter-toolbar {
    display: grid;
    grid-template-columns: auto 1fr auto;
    justify-content: start;
    align-items: center;
    gap: var(--horizontal-gap);
    padding: 0;
    background: var(--background);
  }

  .actions {
    flex: 1;
    display: flex;
    justify-content: start;
  }

  .details {
    flex: 1;
    display: flex;
    justify-content: center;
    align-items: center;
    gap: var(--horizontal-gap);
  }

  .participants {
    flex: 1;
    display: flex;
    justify-content: end;
    align-items: center;
  }

  .round {
    font-weight: bold;

    &::before {
      font-weight: normal;
      content: "Round ";
      color: var(--dim-foreground);
    }
  }

  .time {
    color: var(--dim-foreground);

    &::before {
      font-style: normal;
      content: "| ";
    }
  }

  .titlebar {
    -webkit-app-region: drag;
    align-self: stretch;
  }
</style>
