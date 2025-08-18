import type { Condition } from "$lib/model/Condition";
import type { Lair } from "$lib/model/Participant";
import { ParticipantViewModel } from "./ParticipantViewModel";

export class LairViewModel extends ParticipantViewModel {
  constructor(private _id: number, private _model: Lair) {
    super();
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
}
