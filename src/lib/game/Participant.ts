import type { Stats } from "./Stats";

export interface MonsterDef {
  name: string,
  subtype: string,
  stats: Stats,
  cr: number,
  ac: number,
  initiativeBonus: number,
  legendaryActions: number,
  portrait?: string,
  notes: string,
  hitDice: string,
}

export interface Monster {
  def: MonsterDef,
  name: string,
  initiative: number,
  action: boolean,
  hp: number,
  reaction: boolean,
  bonusAction: boolean,
  legendaryActions: number,
  notes: string,
  tiebreaker: number,
  conditions: string[],
}

export interface PlayerDef {
  name: string,
  classes: { class: string, level: number }[],
  stats: Stats,
  ac: number,
  initiativeBonus: number,
  portrait?: string,
  notes: string,
}

export interface Player {
  def: PlayerDef,
  initiative: number,
  action: boolean,
  reaction: boolean,
  bonus_action: boolean,
  notes: string,
  tiebreaker: number,
  conditions: string[],
}

export interface Lair {
  name: string,
  notes: string,
  portrait?: string,
}

export type Participant = { monster: Monster } | { player: Player } | { lair: Lair };

export function isMonster(participant: Participant): participant is { monster: Monster } {
  return "monster" in participant;
}

export function isPlayer(participant: Participant): participant is { player: Player } {
  return "player" in participant;
}

export function isLair(participant: Participant): participant is { lair: Lair } {
  return "lair" in participant;
}

export function participantName(participant: Participant): string {
  if (isMonster(participant)) {
    return participant.monster.name;
  } else if (isPlayer(participant)) {
    return participant.player.def.name;
  } else if (isLair(participant)) {
    return participant.lair.name;
  }
  return "Unknown";
}

export function portrait(participant: Participant): string {
  if (isMonster(participant)) {
    return participant.monster.def.portrait ?? "unknown-monster";
  } else if (isPlayer(participant)) {
    return participant.player.def.portrait ?? "unknown-player";
  } else if (isLair(participant)) {
    return participant.lair.portrait ?? "lair";
  }

  throw new Error("Unknown participant type");
}

export function smallPortraitUrl(participant: Participant): string {
  return `/images/portraits/${portrait(participant)}.small.jpg`;
}

export function fullPortraitUrl(participant: Participant): string {
  return `/images/portraits/${portrait(participant)}.full.jpg`;
}
