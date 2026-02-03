// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

import { gameCommands } from "$lib/model/Commands";
import type { Condition } from "$lib/model/gen/Condition";
import type { Participant } from "$lib/model/gen/Participant";

export abstract class ParticipantViewModel {
  abstract get model(): Participant;
  abstract get name(): string;
  abstract get smallPortrait(): string;
  abstract get fullPortrait(): string;
  abstract get initiative(): number;
  abstract get conditions(): Condition[];
  abstract get id(): number;

  async addConditions(conditions: Condition[]): Promise<void> {
    await gameCommands.addConditions({
      target: this.id,
      conditions
    });
  }
}

export const conditionPriorities: Record<Condition["name"], number> = {
  "dead": 1,
  "bloodied": 2,
  "unconscious": 3,
  "stunned": 4,
  "concentrating": 5,
  "surprised": 6,
  "frightened": 7,
  "charmed": 8,
  "blinded": 9,
  "deafened": 10,
  "marked": 11,
  "poisoned": 12,
  "invisible": 13,
  "paralyzed": 14,
  "petrified": 15,
  "prone": 16,
  "grappled": 17,
  "incapacitated": 18,
  "restrained": 19,
};
