<!--
  Copyright (c) 2025 Jonathon B. Cobb
  Licensed under the MIT License
-->

<script lang="ts">
  import {
    calculateEncounterDifficulty,
    type DifficultyThresholds51,
    type DifficultyThresholds52,
    type EncounterDifficulty,
  } from "$lib/model/Difficulty";
  import { crString, isMonster, isPlayer, type Participant } from "$lib/model/Participant";
  import { currentRulesVersion } from "$lib/theme";

  export let participants: Participant[] = [];

  $: difficulty = calculateDifficulty(participants, $currentRulesVersion);

  function calculateDifficulty(participants: Participant[], rulesVersion: string): EncounterDifficulty {
    const monsterCrs = participants.filter(isMonster).map((m) => m.cr);
    const playerLevels = participants.filter(isPlayer).map((p) => p.classes.reduce((sum, c) => sum + c.level, 0));

    return calculateEncounterDifficulty(monsterCrs, playerLevels, rulesVersion as "srd51" | "srd52");
  }

  function formatNumber(n: number): string {
    return n.toLocaleString();
  }

  function formatCr(avgCr: number): string {
    const rounded = Math.round(avgCr);
    return crString(rounded);
  }

  // Type guards for thresholds
  function is51Thresholds(t: DifficultyThresholds51 | DifficultyThresholds52): t is DifficultyThresholds51 {
    return "deadly" in t;
  }

  function getMaxThreshold(difficulty: EncounterDifficulty): number {
    if (is51Thresholds(difficulty.thresholds)) {
      return difficulty.thresholds.deadly;
    } else {
      return difficulty.thresholds.high;
    }
  }

  function fillPercent(difficulty: EncounterDifficulty): number {
    const max = getMaxThreshold(difficulty);
    if (max === 0) {
      return 0;
    } else {
      return Math.min(100, (difficulty.adjustedXp / max) * 100);
    }
  }

  function thresholdPercent(threshold: number, max: number): number {
    if (max === 0) {
      return 0;
    } else {
      return (threshold / max) * 100;
    }
  }
</script>

<div class="difficulty-gauge">
  <div class="gauge">
    {#if difficulty.rulesVersion === "srd51" && is51Thresholds(difficulty.thresholds)}
      {@const t = difficulty.thresholds}
      {@const max = t.deadly}
      <div class="labels">
        <span class="label threshold" class:active={difficulty.adjustedXp > 0} style="left: 0%">0</span>
        <span
          class="label trivial"
          class:active={difficulty.rating === "trivial"}
          style="left: {thresholdPercent(t.easy / 2, max)}%"
        >
          Trivial
        </span>
        <span
          class="label threshold"
          class:active={difficulty.adjustedXp >= t.easy}
          style="left: {thresholdPercent(t.easy, max)}%"
        >
          {formatNumber(t.easy)}
        </span>
        <span
          class="label easy"
          class:active={difficulty.rating === "easy"}
          style="left: {thresholdPercent((t.easy + t.medium) / 2, max)}%"
        >
          Easy
        </span>
        <span
          class="label threshold"
          class:active={difficulty.adjustedXp >= t.medium}
          style="left: {thresholdPercent(t.medium, max)}%"
        >
          {formatNumber(t.medium)}
        </span>
        <span
          class="label medium"
          class:active={difficulty.rating === "medium"}
          style="left: {thresholdPercent((t.medium + t.hard) / 2, max)}%"
        >
          Medium
        </span>
        <span
          class="label threshold"
          class:active={difficulty.adjustedXp >= t.hard}
          style="left: {thresholdPercent(t.hard, max)}%"
        >
          {formatNumber(t.hard)}
        </span>
        <span
          class="label hard"
          class:active={difficulty.rating === "hard"}
          style="left: {thresholdPercent((t.hard + t.deadly) / 2, max)}%"
        >
          Hard
        </span>
        <span class="label threshold" class:active={difficulty.adjustedXp >= t.deadly} style="left: 100%">
          {formatNumber(t.deadly)}
        </span>
      </div>

      <div class="bar">
        <div class="fill {difficulty.rating}" style="width: {fillPercent(difficulty)}%"></div>
        <div class="marker" style="left: {thresholdPercent(t.easy, max)}%"></div>
        <div class="marker" style="left: {thresholdPercent(t.medium, max)}%"></div>
        <div class="marker" style="left: {thresholdPercent(t.hard, max)}%"></div>
      </div>
    {:else if !is51Thresholds(difficulty.thresholds)}
      {@const t = difficulty.thresholds}
      {@const max = t.high}
      <div class="labels">
        <span class="label threshold" class:active={difficulty.totalXp > 0} style="left: 0%">0</span>
        <span
          class="label trivial"
          class:active={difficulty.rating === "trivial"}
          style="left: {thresholdPercent(t.low / 2, max)}%"
        >
          Trivial
        </span>
        <span
          class="label threshold"
          class:active={difficulty.totalXp >= t.low}
          style="left: {thresholdPercent(t.low, max)}%"
        >
          {formatNumber(t.low)}
        </span>
        <span
          class="label low"
          class:active={difficulty.rating === "low"}
          style="left: {thresholdPercent((t.low + t.moderate) / 2, max)}%"
        >
          Low
        </span>
        <span
          class="label threshold"
          class:active={difficulty.totalXp >= t.moderate}
          style="left: {thresholdPercent(t.moderate, max)}%"
        >
          {formatNumber(t.moderate)}
        </span>
        <span
          class="label moderate"
          class:active={difficulty.rating === "moderate"}
          style="left: {thresholdPercent((t.moderate + t.high) / 2, max)}%"
        >
          Moderate
        </span>
        <span class="label threshold" class:active={difficulty.totalXp >= t.high} style="left: 100%">
          {formatNumber(t.high)}
        </span>
      </div>

      <div class="bar">
        <div class="fill {difficulty.rating}" style="width: {fillPercent(difficulty)}%"></div>
        <div class="marker" style="left: {thresholdPercent(t.low, max)}%"></div>
        <div class="marker" style="left: {thresholdPercent(t.moderate, max)}%"></div>
      </div>
    {/if}
  </div>

  <div class="stats">
    <div class="stat">
      <span class="value">{formatNumber(difficulty.totalXp)}</span>
      <span class="label">XP</span>
      {#if difficulty.rulesVersion === "srd51"}
        <span class="detail">{formatNumber(difficulty.adjustedXp)} Adjusted XP</span>
      {/if}
    </div>
    <div class="stat">
      <span class="value">{difficulty.playerCount}</span>
      <span class="label">Players</span>
      <span class="detail">{formatNumber(difficulty.xpPerPlayer)} XP / Player</span>
    </div>
    <div class="stat">
      <span class="value">{difficulty.monsterCount}</span>
      <span class="label">Monsters</span>
      <span class="detail">Average CR {formatCr(difficulty.averageCr)}</span>
    </div>
  </div>
</div>

<style lang="scss">
  .difficulty-gauge {
    padding: var(--vertical-gap) calc(var(--horizontal-gap) * 8);
  }

  .gauge {
    position: relative;
    margin-bottom: calc(var(--vertical-gap) * 2);
  }

  .labels {
    position: relative;
    height: 1.25rem;
    margin-bottom: var(--vertical-gap);

    .label {
      position: absolute;
      transform: translateX(-50%);
      color: var(--dim-foreground);
      white-space: nowrap;

      &.active {
        font-weight: bold;

        &.trivial,
        &.easy,
        &.low {
          color: var(--ok-foreground);
        }

        &.medium,
        &.moderate {
          color: var(--caution-foreground);
        }

        &.hard,
        &.deadly,
        &.high {
          color: var(--danger-foreground);
        }
      }

      &.threshold.active {
        color: var(--foreground);
        font-weight: bold;
      }
    }
  }

  .bar {
    position: relative;
    height: 0.75rem;
    background: var(--card-background);
    border: 1px solid var(--chrome);
    border-radius: var(--panel-border-radius);
    overflow: hidden;

    .fill {
      height: 100%;
      transition: width var(--ui-transition-time) ease-out;

      &.trivial,
      &.easy,
      &.low {
        background: var(--ok-foreground);
      }

      &.medium,
      &.moderate {
        background: var(--caution-foreground);
      }

      &.hard,
      &.deadly,
      &.high {
        background: var(--danger-foreground);
      }
    }

    .marker {
      position: absolute;
      top: 0;
      bottom: 0;
      width: 1px;
      background: var(--dim-foreground);
    }
  }

  .stats {
    display: flex;
    gap: 2rem;

    .stat {
      align-items: first;
      display: grid;
      grid-template-columns: auto auto;
      grid-template-rows: auto auto;
      gap: 0 var(--horizontal-gap);

      .value {
        grid-row: 1 / span 2;
        grid-column: 1;
        font-size: calc(var(--base-font-size) * 2);
        margin-top: -0.25rem;
      }

      .label {
        grid-row: 1;
        grid-column: 2;
      }

      .detail {
        grid-row: 2;
        grid-column: 2;
        color: var(--dim-foreground);
        font-style: italic;
      }
    }
  }
</style>
