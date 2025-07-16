<script lang="ts">
  import type { GameViewModel } from "$lib/viewmodel/GameViewModel";
  import ParticipantRow from "./encounter/ParticipantRow.svelte";

  export let game: GameViewModel;
</script>

<div>
  {#each game.participants
    .slice()
    .sort((a, b) => b.initiative - a.initiative) as participant}
    <ParticipantRow
      {participant}
      isActive={participant.initiative == 15}
      isSelected={false}
    />
  {/each}
</div>

<style lang="scss">
  div {
    display: flex;
    flex-direction: column;
    gap: var(--vertical-gap);
  }
</style>
