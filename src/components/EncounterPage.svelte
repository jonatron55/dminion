<script lang="ts">
  import type { GameViewModel } from "$lib/viewmodel/GameViewModel";
  import ParticipantRow from "./encounter/ParticipantRow.svelte";

  export let game: GameViewModel;
</script>

<main>
  <div class="participants">
    {#each Object.entries(game.participants)
      .slice()
      .sort((a, b) => b[1].initiative - a[1].initiative) as [id, participant]}
      <ParticipantRow {id} {participant} {game} isActive={participant.initiative == 15} />
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
    padding: 0 var(--horizontal-gap);
  }
</style>
