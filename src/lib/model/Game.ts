import type { Participant } from "./Participant";

export interface Game {
  participants: Participant[];
  round: number,
  turn: number,
  game_started: Date,
  turn_started: Date,
}
