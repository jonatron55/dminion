// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

import type { Participant } from "./Participant";

export interface Game {
  participants: Record<number, Participant>;
  order: number[];
  round: number;
  turn: number;
  gameStarted: Date;
  turnStarted: Date;
}
