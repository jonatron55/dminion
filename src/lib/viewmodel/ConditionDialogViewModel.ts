// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

import type { Condition, Expiry } from "$lib/model/Condition";
import { SecondsPerRound } from "$lib/Time";
import type { GameViewModel } from "$lib/viewmodel/GameViewModel";
import type { ParticipantViewModel } from "$lib/viewmodel/ParticipantViewModel";

type DurationType = "untilRemoved" | "elapsed" | "startTurn" | "endTurn";
type ElapsedUnit = "seconds" | "minutes" | "hours";

export interface ConditionBuildOptions {
  selectedConditions: string[];
  customConditionName: string;
  durationType: DurationType;
  elapsedDuration: number;
  elapsedUnit: ElapsedUnit;
  instigatorId: number | null;
}

export class ConditionDialogViewModel {
  constructor(private readonly game: GameViewModel, private readonly participant: ParticipantViewModel) { }

  get participants(): ParticipantViewModel[] {
    return Object.values(this.game.participants);
  }

  getDefaultInstigatorId(): number | null {
    if (this.participant.id === this.game.activeParticipantId) {
      return null;
    }

    return this.game.activeParticipantId ?? null;
  }

  getTurnParticipantId(instigatorId: number | null): number {
    return instigatorId ?? this.participant.id;
  }

  getParticipantName(participantId: number): string {
    return this.game.participants[participantId]?.name ?? this.participant.name;
  }

  buildConditions(options: ConditionBuildOptions): Condition[] {
    const { selectedConditions, customConditionName, durationType, elapsedDuration, elapsedUnit, instigatorId } =
      options;

    const trimmedCustom = customConditionName.trim();
    const baseConditions = selectedConditions.filter((condition) => condition !== "other");
    const finalNames =
      selectedConditions.includes("other") && trimmedCustom ? [...baseConditions, trimmedCustom] : baseConditions;

    const expiry = this.toExpiry(durationType, elapsedDuration, elapsedUnit);
    const startTime = { round: this.game.round, initiative: this.game.initiative };

    return finalNames.map<Condition>((name) => ({
      name,
      startTime,
      expiry,
      instigator: instigatorId ?? undefined,
    }));
  }

  private toExpiry(durationType: DurationType, elapsedDuration: number, elapsedUnit: ElapsedUnit): Expiry {
    switch (durationType) {
      case "startTurn":
        return { type: "nextTurnStart" };
      case "endTurn":
        return { type: "nextTurnEnd" };
      case "elapsed": {
        const seconds = this.toSeconds(elapsedDuration, elapsedUnit);
        const rounds = Math.max(1, Math.ceil(seconds / SecondsPerRound));
        return { type: "duration", rounds };
      }
      case "untilRemoved":
      default:
        return { type: "none" };
    }
  }

  private toSeconds(amount: number, unit: ElapsedUnit): number {
    switch (unit) {
      case "hours":
        return amount * 3600;
      case "minutes":
        return amount * 60;
      case "seconds":
      default:
        return amount;
    }
  }
}
