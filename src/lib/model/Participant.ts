import type { Condition } from "./Condition";
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
  tempHp: number,
  maxHp: number,
  reaction: boolean,
  bonusAction: boolean,
  legendaryActions: number,
  notes: string,
  tiebreaker: number,
  conditions: Condition[],
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
  bonusAction: boolean,
  notes: string,
  tiebreaker: number,
  conditions: Condition[],
}

export interface Lair {
  name: string,
  action: boolean,
  notes: string,
  portrait?: string,
}

export type Participant = { monster: Monster } | { player: Player } | { lair: Lair };
