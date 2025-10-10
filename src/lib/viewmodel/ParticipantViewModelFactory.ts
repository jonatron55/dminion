// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

import type { Participant } from "$lib/model/Participant";
import { isLair, isMonster, isPlayer } from "$lib/model/Participant";
import { LairViewModel } from "./LairViewModel";
import { MonsterViewModel } from "./MonsterViewModel";
import type { ParticipantViewModel } from "./ParticipantViewModel";
import { PlayerViewModel } from "./PlayerViewModel";

export function createParticipantViewModel(id: number, model: Participant): ParticipantViewModel {
  if (isMonster(model)) {
    return new MonsterViewModel(id, model);
  } else if (isPlayer(model)) {
    return new PlayerViewModel(id, model);
  } else if (isLair(model)) {
    return new LairViewModel(id, model);
  } else {
    throw new Error(`Unknown participant type '${typeof model}' for id ${id}`);
  }
}
