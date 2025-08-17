import type { Condition } from "./Condition";
import type { Stats } from "./Stats";

export interface Monster {
  type: "monster",
  name: string,
  subtype: string,
  stats: Stats,
  cr: number,
  ac: number,
  initiativeBonus: number,
  smallPortrait?: string,
  fullPortrait?: string,
  hitDice: string,
  initiative: number,
  action: boolean,
  hp: number,
  tempHp: number,
  maxHp: number,
  reaction: boolean,
  bonusAction: boolean,
  legendaryActions: number,
  notes: string,
  tiebreaker: number,
  conditions: Condition[],
}

export interface Player {
  type: "player",
  name: string,
  classes: { class: string, level: number }[],
  stats: Stats,
  ac: number,
  initiativeBonus: number,
  smallPortrait?: string,
  fullPortrait?: string,
  initiative: number,
  action: boolean,
  reaction: boolean,
  bonusAction: boolean,
  notes: string,
  tiebreaker: number,
  conditions: Condition[],
}

export interface Lair {
  type: "lair",
  name: string,
  action: boolean,
  notes: string,
  smallPortrait?: string,
  fullPortrait?: string,
}

export type Participant = Monster | Player | Lair;

export function isMonster(participant: Participant): participant is Monster {
  return participant.type === "monster";
}

export function isPlayer(participant: Participant): participant is Player {
  return participant.type === "player";
}

export function isLair(participant: Participant): participant is Lair {
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
