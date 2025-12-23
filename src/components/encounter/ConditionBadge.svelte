<!--
  Copyright (c) 2025 Jonathon B. Cobb
  Licensed under the MIT License
-->

<script lang="ts">
  import type { Condition } from "$lib/model/Condition";
  import { formatDuration } from "$lib/Time";
  import type { GameViewModel } from "$lib/viewmodel/GameViewModel";

  export let condition: Condition;
  export let game: GameViewModel;

  $: remainingRounds =
    condition.expiry.type === "duration"
      ? Math.max(0, condition.startTime.round + condition.expiry.rounds - game.round)
      : null;
  $: remainingLabel = remainingRounds ? formatDuration(remainingRounds) : null;
  $: instigatorName = condition.instigator ? game.participants[condition.instigator]?.name : null;
</script>

<span class="{condition.name} badge">
  <span>{condition.name}</span>
  {#if instigatorName}
    <span class="instigator">by {instigatorName}</span>
  {/if}
  {#if remainingLabel}
    <span class="count">{remainingLabel}</span>
  {/if}
</span>

<style lang="scss">
  .instigator {
    text-transform: none;
  }
</style>
