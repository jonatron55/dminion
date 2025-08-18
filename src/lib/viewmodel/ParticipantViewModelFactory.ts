import type { Participant } from "$lib/model/Participant";
import { isLair, isMonster, isPlayer } from "$lib/model/Participant";
import { LairViewModel } from "./LairViewModel";
import { MonsterViewModel } from "./MonsterViewModel";
import type { ParticipantViewModel } from "./ParticipantViewModel";
import { PlayerViewModel } from "./PlayerViewModel";

export function createParticipantViewModel(model: Participant): ParticipantViewModel {
  if (isMonster(model)) {
    return new MonsterViewModel(model);
  } else if (isPlayer(model)) {
    return new PlayerViewModel(model);
  } else if (isLair(model)) {
    return new LairViewModel(model);
  } else {
    throw new Error("Unknown participant type");
  }
}
