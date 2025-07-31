import type { Participant } from "./Participant";

export interface Game {
  participants: Participant[];
  round: number,
  turn: number,
  turnStarted: Date,
}
