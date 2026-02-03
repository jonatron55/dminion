// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

import type { Condition } from "./gen/Condition";

export const conditionNames: string[] = [
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
