<script lang="ts">
  import type { GameViewModel } from "$lib/viewmodel/GameViewModel";
  import ParticipantRow from "./encounter/ParticipantRow.svelte";

  export let game: GameViewModel;
</script>

<main>
  <div class="participants">
    {#each Array(4).fill(0) as _}
      {#each game.participants
        .slice()
        .sort((a, b) => b.initiative - a.initiative) as participant}
        <ParticipantRow
          {participant}
          isActive={participant.initiative == 15}
          isSelected={false}
        />
      {/each}
    {/each}
  </div>
</main>

<style lang="scss">
  main {
    display: grid;
    grid-template-rows: 1fr auto;
    overflow: hidden;
  }

  .participants {
    grid-row: 1;
    display: flex;
    flex-direction: column;
    gap: var(--vertical-gap);
    overflow-y: scroll;
  }
</style>
