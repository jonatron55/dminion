<script lang="ts">
  import type { GameViewModel } from "$lib/viewmodel/GameViewModel";

  export let game: GameViewModel;

  function time(): string {
    const minutes = Math.floor(game.time / 60);
    const seconds = game.time % 60;
    return `${minutes}:${seconds < 10 ? "0" : ""}${seconds}`;
  }
</script>

<div class="encounter-toolbar">
  <span class="actions">
    <button class="toolbar">↺ Undo</button>
    <button class="toolbar">⏩ Next</button>
  </span>
  <span data-tauri-drag-region class="titlebar details">
    <span data-tauri-drag-region class="titlebar round">{game.round}</span>
    <span data-tauri-drag-region class="titlebar time">{time()}</span>
  </span>
  <span class="participants">
    <button class="toolbar">+ Player</button>
    <button class="toolbar">+ Monster</button>
  </span>
</div>

<style lang="scss">
  .encounter-toolbar {
    display: flex;
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
</style>
