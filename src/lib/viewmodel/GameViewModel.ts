import type { Game } from "$lib/model/Game";
import { ParticipantViewModel } from "./ParticipantViewModel";

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

  constructor(private _model: Game) {
    this._participants = Object.fromEntries(
      Object.entries(this._model.participants).map(
        ([id, participant]) => [Number(id), ParticipantViewModel.create(participant)]
      )
    );
  }
}
