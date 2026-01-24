// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

import { gameCommands } from "$lib/model/Commands";
import type { Condition } from "$lib/model/Condition";
import type { Action, Lair } from "$lib/model/Participant";
import { ParticipantViewModel } from "./ParticipantViewModel";

export class LairViewModel extends ParticipantViewModel {
  constructor(private _id: number, private _model: Lair) {
    super();
  }

  get model(): Lair {
    return this._model;
  }

  get id(): number {
    return this._id;
  }

  get name(): string {
    return this._model.name;
  }

  get action(): boolean {
    return this._model.action;
  }

  set action(value: boolean) {
    this._model.action = value;
    this.setAction({ type: "standard" }, value);
  }

  get smallPortrait(): string {
    return this._model.smallPortrait || "/images/portraits/lair.small.jpg";
  }

  get fullPortrait(): string {
    return this._model.fullPortrait || "/images/portraits/lair.full.jpg";
  }

  get conditions(): Condition[] {
    return [];
  }

  get initiative(): number {
    return 20;
  }

  private async setAction(action: Action, available: boolean): Promise<void> {
    await gameCommands.setAction({
      target: this._id,
      action,
      available
    });
  }
}
