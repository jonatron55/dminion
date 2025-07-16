import type { Game } from "$lib/model/Game";
import { ParticipantViewModel } from "./ParticipantViewModel";

export class GameViewModel {
  private _participants: ParticipantViewModel[] = [];

  public get participants(): ParticipantViewModel[] {
    return this._participants;
  }

  constructor(private _model: Game) {
    this._participants = this._model.participants.map(ParticipantViewModel.create);
  }
}
