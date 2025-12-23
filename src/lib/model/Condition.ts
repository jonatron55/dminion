// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

import type { GameTime } from "./Game";

export interface Condition {
  name: string;
  startTime: GameTime,
  expiry: Expiry,
  instigator?: number,
}

export type Expiry = {
  type: "none";
} | {
  type: "nextTurnStart";
} | {
  type: "nextTurnEnd";
} | {
  type: "duration";
  rounds: number;
}

export const conditionNames: Condition["name"][] = [
  "blinded",
  "bloodied",
  "charmed",
  "concentrating",
  "dead",
  "deafened",
  "frightened",
  "grappled",
  "incapacitated",
  "invisible",
  "marked",
  "paralyzed",
  "petrified",
  "poisoned",
  "prone",
  "restrained",
  "stunned",
  "surprised",
  "unconscious"
];

export function conditionClasses(conditions: Condition[]): string {
  return conditions
    .map(c => c.name)
    .filter(c => conditionNames.includes(c))
    .join(" ");
}
