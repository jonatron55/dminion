// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

// Re-export from generated types
export type { Stats } from "./gen/Stats";

export function modifer(score: number): number {
  if (score >= 10) {
    return Math.floor((score - 10) / 2);
  } else {
    return Math.floor((score - 11) / 2);
  }
}

export function statStr(score: number): string {
  return score < 10 ? `\u00A0${score} ` : `${score} `;
}

export function modiferStr(score: number): string {
  const mod = modifer(score);
  return mod >= 0 ? `+${mod}` : `${mod}`;
}
