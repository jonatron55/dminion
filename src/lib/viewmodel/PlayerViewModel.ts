// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

import { gameCommands } from "$lib/model/Commands";
import type { Condition } from "$lib/model/Condition";
import type { Action, Player } from "$lib/model/Participant";
import type { Stats } from "$lib/model/Stats";
import { ParticipantViewModel, conditionPriorities } from "./ParticipantViewModel";

export class PlayerViewModel extends ParticipantViewModel {
  constructor(private _id: number, private _model: Player) {
    super();
  }

  get id(): number {
    return this._id;
  }

  get name(): string {
    return this._model.name;
  }

  get smallPortrait(): string {
    return this._model.smallPortrait ?? "/images/portraits/unknown-player.small.jpg";
  }

  get fullPortrait(): string {
    return this._model.fullPortrait ?? "/images/portraits/unknown-player.full.jpg";
  }

  get initiative(): number {
    return this._model.initiative;
  }

  get conditions(): Condition[] {
    return this._model.conditions.slice().map(s => s).sort((a, b) =>
      (conditionPriorities[a.name] ?? 1000) - (conditionPriorities[b.name] ?? 1000)
    );
  }

  get stats(): Stats {
    return this._model.stats;
  }

  get action(): boolean {
    return this._model.action;
  }

  set action(value: boolean) {
    this._model.action = value;
    this.setAction({ type: "standard" }, value);
  }

  get reaction(): boolean {
    return this._model.reaction;
  }

  set reaction(value: boolean) {
    this._model.reaction = value;
    this.setAction({ type: "reaction" }, value);
  }

  get bonusAction(): boolean {
    return this._model.bonusAction;
  }

  set bonusAction(value: boolean) {
    this._model.bonusAction = value;
    this.setAction({ type: "bonus" }, value);
  }

  private async setAction(action: Action, available: boolean): Promise<void> {
    await gameCommands.setAction({
      target: this._id,
      action,
      available
    });
  }
}
