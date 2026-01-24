<!--
  Copyright (c) 2025 Jonathon B. Cobb
  Licensed under the MIT License
-->

<script lang="ts">
  import type { GameViewModel } from "$lib/viewmodel/GameViewModel";
  import DifficultyGauge from "./DifficultyGauge.svelte";
  import ParticipantRow from "./ParticipantRow.svelte";

  export let game: GameViewModel;

  $: participantList = Object.values(game.participants).map((vm) => vm.model);
</script>

<main>
  <div class="participants">
    {#each Object.entries(game.participants)
      .slice()
      .sort((a, b) => b[1].initiative - a[1].initiative) as [, participant]}
      <ParticipantRow {participant} {game} isActive={game.activeParticipantId === participant.id} />
    {/each}
  </div>
  <DifficultyGauge participants={participantList} />
</main>

<style lang="scss">
  main {
    height: 100%;
    display: grid;
    grid-template-rows: 1fr auto;
    overflow: hidden;
  }

  .participants {
    grid-row: 1;
    display: flex;
    flex-direction: column;
    gap: var(--vertical-gap);
    overflow-y: auto;
    padding: 0 var(--horizontal-gap);
  }
</style>
