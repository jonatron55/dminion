// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

import type { RulesVersion } from "$lib/theme";

/**
 * XP values indexed by CR encoding (0 = CR 0, 1 = CR 1/8, 2 = CR 1/4, 3 = CR 1/2, 4+ = CR n-3).
 */
export const XP_PER_CR: readonly number[] = [
  10, 25, 50, 100, 200, 450, 700, 1100, 1800, 2300, 2900, 3900, 5000, 5900, 7200, 8400, 10000,
  11500, 13000, 15000, 18000, 20000, 22000, 25000, 33000, 41000, 50000, 62000, 75000, 90000,
  105000, 120000, 135000, 155000,
];

/**
 * SRD 5.1 difficulty thresholds per player level (1-indexed, index 0 unused).
 * Thresholds are summed across all party members.
 */
export const SRD51_THRESHOLDS: readonly { easy: number; medium: number; hard: number; deadly: number }[] = [
  { easy: 0, medium: 0, hard: 0, deadly: 0 }, // Index 0 (unused)
  { easy: 25, medium: 50, hard: 75, deadly: 100 },
  { easy: 50, medium: 100, hard: 150, deadly: 200 },
  { easy: 75, medium: 150, hard: 225, deadly: 400 },
  { easy: 125, medium: 250, hard: 375, deadly: 500 },
  { easy: 250, medium: 500, hard: 750, deadly: 1100 },
  { easy: 300, medium: 600, hard: 900, deadly: 1400 },
  { easy: 350, medium: 750, hard: 1100, deadly: 1700 },
  { easy: 450, medium: 900, hard: 1400, deadly: 2100 },
  { easy: 550, medium: 1100, hard: 1600, deadly: 2400 },
  { easy: 600, medium: 1200, hard: 1900, deadly: 2800 },
  { easy: 800, medium: 1600, hard: 2400, deadly: 3600 },
  { easy: 1000, medium: 2000, hard: 3000, deadly: 4500 },
  { easy: 1100, medium: 2200, hard: 3400, deadly: 5100 },
  { easy: 1250, medium: 2500, hard: 3800, deadly: 5700 },
  { easy: 1400, medium: 2800, hard: 4300, deadly: 6400 },
  { easy: 1600, medium: 3200, hard: 4800, deadly: 7200 },
  { easy: 2000, medium: 3900, hard: 5900, deadly: 8800 },
  { easy: 2100, medium: 4200, hard: 6300, deadly: 9500 },
  { easy: 2400, medium: 4900, hard: 7300, deadly: 10900 },
  { easy: 2800, medium: 5700, hard: 8500, deadly: 12700 },
];

/**
 * SRD 5.2 XP budget per character by level (1-indexed, index 0 unused).
 * Budget is multiplied by party size to get total XP budget.
 */
export const SRD52_BUDGETS: readonly { low: number; moderate: number; high: number }[] = [
  { low: 0, moderate: 0, high: 0 }, // Index 0 (unused)
  { low: 50, moderate: 75, high: 100 },
  { low: 100, moderate: 150, high: 200 },
  { low: 150, moderate: 225, high: 400 },
  { low: 250, moderate: 375, high: 500 },
  { low: 500, moderate: 750, high: 1100 },
  { low: 600, moderate: 1000, high: 1400 },
  { low: 750, moderate: 1300, high: 1700 },
  { low: 1000, moderate: 1700, high: 2100 },
  { low: 1300, moderate: 2000, high: 2600 },
  { low: 1600, moderate: 2300, high: 3100 },
  { low: 1900, moderate: 2900, high: 4100 },
  { low: 2200, moderate: 3700, high: 4700 },
  { low: 2600, moderate: 4200, high: 5400 },
  { low: 2900, moderate: 4900, high: 6200 },
  { low: 3300, moderate: 5400, high: 7800 },
  { low: 3800, moderate: 6100, high: 9800 },
  { low: 4500, moderate: 7200, high: 11700 },
  { low: 5000, moderate: 8700, high: 14200 },
  { low: 5500, moderate: 10700, high: 17200 },
  { low: 6400, moderate: 13200, high: 22000 },
];

/** SRD 5.1 uses 4 tiers: trivial, easy, medium, hard, deadly */
export type DifficultyRating51 = "trivial" | "easy" | "medium" | "hard" | "deadly";

/** SRD 5.2 uses 3 tiers: trivial, low, moderate, high */
export type DifficultyRating52 = "trivial" | "low" | "moderate" | "high";

export type DifficultyRating = DifficultyRating51 | DifficultyRating52;

export interface DifficultyThresholds51 {
  easy: number;
  medium: number;
  hard: number;
  deadly: number;
}

export interface DifficultyThresholds52 {
  low: number;
  moderate: number;
  high: number;
}

export type DifficultyThresholds = DifficultyThresholds51 | DifficultyThresholds52;

export interface EncounterDifficulty {
  rulesVersion: RulesVersion;
  totalXp: number;
  adjustedXp: number;
  thresholds: DifficultyThresholds;
  rating: DifficultyRating;
  playerCount: number;
  monsterCount: number;
  xpPerPlayer: number;
  averageCr: number;
}

/** Returns XP for a given CR encoding. */
export function xpForCr(cr: number): number {
  return XP_PER_CR[Math.min(cr, XP_PER_CR.length - 1)];
}

/** Returns the encounter multiplier based on monster count (SRD 5.1 only). */
export function encounterMultiplier(monsterCount: number): number {
  if (monsterCount <= 1) { return 1.0; }
  if (monsterCount === 2) { return 1.5; }
  if (monsterCount <= 6) { return 2.0; }
  if (monsterCount <= 10) { return 2.5; }
  if (monsterCount <= 14) { return 3.0; }
  return 4.0;
}

/** Calculates SRD 5.1 party difficulty thresholds by summing per-player thresholds. */
export function partyThresholds51(playerLevels: number[]): DifficultyThresholds51 {
  const result = { easy: 0, medium: 0, hard: 0, deadly: 0 };

  for (const level of playerLevels) {
    const clampedLevel = Math.max(1, Math.min(level, 20));
    const thresholds = SRD51_THRESHOLDS[clampedLevel];
    result.easy += thresholds.easy;
    result.medium += thresholds.medium;
    result.hard += thresholds.hard;
    result.deadly += thresholds.deadly;
  }

  return result;
}

/** Calculates SRD 5.2 party XP budgets by averaging levels and multiplying by party size. */
export function partyThresholds52(playerLevels: number[]): DifficultyThresholds52 {
  if (playerLevels.length === 0) {
    return { low: 0, moderate: 0, high: 0 };
  }

  const avgLevel = Math.round(playerLevels.reduce((sum, l) => sum + l, 0) / playerLevels.length);
  const clampedLevel = Math.max(1, Math.min(avgLevel, 20));
  const budgets = SRD52_BUDGETS[clampedLevel];

  return {
    low: budgets.low * playerLevels.length,
    moderate: budgets.moderate * playerLevels.length,
    high: budgets.high * playerLevels.length,
  };
}

/** Determines SRD 5.1 difficulty rating based on adjusted XP vs thresholds. */
export function difficultyRating51(adjustedXp: number, thresholds: DifficultyThresholds51): DifficultyRating51 {
  if (adjustedXp >= thresholds.deadly) { return "deadly"; }
  if (adjustedXp >= thresholds.hard) { return "hard"; }
  if (adjustedXp >= thresholds.medium) { return "medium"; }
  if (adjustedXp >= thresholds.easy) { return "easy"; }
  return "trivial";
}

/** Determines SRD 5.2 difficulty rating based on total XP vs budgets. */
export function difficultyRating52(totalXp: number, thresholds: DifficultyThresholds52): DifficultyRating52 {
  if (totalXp >= thresholds.high) { return "high"; }
  if (totalXp >= thresholds.moderate) { return "moderate"; }
  if (totalXp >= thresholds.low) { return "low"; }
  return "trivial";
}

/** Calculates full encounter difficulty metrics using specified rules version. */
export function calculateEncounterDifficulty(
  monsterCrs: number[],
  playerLevels: number[],
  rulesVersion: RulesVersion = "srd51"
): EncounterDifficulty {
  const totalXp = monsterCrs.reduce((sum, cr) => sum + xpForCr(cr), 0);
  const playerCount = playerLevels.length;
  const monsterCount = monsterCrs.length;
  const xpPerPlayer = playerCount > 0 ? Math.floor(totalXp / playerCount) : 0;
  const averageCr = monsterCount > 0
    ? monsterCrs.reduce((sum, cr) => sum + cr, 0) / monsterCount
    : 0;

  if (rulesVersion === "srd52") {
    // SRD 5.2: No encounter multiplier, compare raw XP to budget
    const thresholds = partyThresholds52(playerLevels);
    const rating = difficultyRating52(totalXp, thresholds);

    return {
      rulesVersion,
      totalXp,
      adjustedXp: totalXp, // No adjustment in 5.2
      thresholds,
      rating,
      playerCount,
      monsterCount,
      xpPerPlayer,
      averageCr,
    };
  } else {
    // SRD 5.1: Apply encounter multiplier
    const multiplier = encounterMultiplier(monsterCount);
    const adjustedXp = Math.ceil(totalXp * multiplier);
    const thresholds = partyThresholds51(playerLevels);
    const rating = difficultyRating51(adjustedXp, thresholds);

    return {
      rulesVersion,
      totalXp,
      adjustedXp,
      thresholds,
      rating,
      playerCount,
      monsterCount,
      xpPerPlayer,
      averageCr,
    };
  }
}
