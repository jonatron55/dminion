// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

import type { Game } from "$lib/model/Game";
import type { ParticipantViewModel } from "./ParticipantViewModel";
import { createParticipantViewModel } from "./ParticipantViewModelFactory";

export class GameViewModel {
  private _participants: Record<number, ParticipantViewModel> = {};

  public get participants(): Record<number, ParticipantViewModel> {
    return this._participants;
  }

  public get round(): number {
    return this._model.round;
  }

  public get turn(): number {
    return this._model.turn;
  }

  public get time(): number {
    return this._model.round * 6;
  }

  public get activeParticipantId(): number {
    return this._model.order[this._model.turn];
  }

  constructor(private _model: Game) {
    this._participants = Object.fromEntries(
      Object.entries(this._model.participants).map(
        ([id, participant]) => [Number(id), createParticipantViewModel(Number(id), participant)]
      )
    );
  }
}
