// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

import type { Lair } from "./gen/Lair";
import type { Monster } from "./gen/Monster";
import type { Participant } from "./gen/Participant";
import type { Player } from "./gen/Player";

// Type guards
export function isMonster(participant: Participant): participant is Monster & { type: "monster" } {
  return participant.type === "monster";
}

export function isPlayer(participant: Participant): participant is Player & { type: "player" } {
  return participant.type === "player";
}

export function isLair(participant: Participant): participant is Lair & { type: "lair" } {
  return participant.type === "lair";
}

export function crValue(cr: number): number {
  if (cr === 0) {
    return 0.0;
  } else if (cr === 1) {
    return 0.125;
  } else if (cr === 2) {
    return 0.25;
  } else if (cr === 3) {
    return 0.5;
  } else {
    return cr - 3.0;
  }
}

export function crString(cr: number): string {
  if (cr === 0) {
    return "0";
  } else if (cr === 1) {
    return "⅛";
  } else if (cr === 2) {
    return "¼";
  } else if (cr === 3) {
    return "½";
  } else {
    return `${cr - 3}`;
  }
}
