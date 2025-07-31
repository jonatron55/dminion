import type { Game } from "$lib/model/Game";
import { ParticipantViewModel } from "./ParticipantViewModel";

export class GameViewModel {
  private _participants: ParticipantViewModel[] = [];

  public get participants(): ParticipantViewModel[] {
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
    this._participants = this._model.participants.map(ParticipantViewModel.create);
  }
}
