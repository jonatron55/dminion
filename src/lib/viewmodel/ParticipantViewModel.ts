import type { Condition } from "$lib/model/Condition";
import type {
  Lair,
  Monster,
  Participant,
  Player
} from "$lib/model/Participant";
import type { Stats } from "$lib/model/Stats";

export abstract class ParticipantViewModel {
  abstract get name(): string;
  abstract get smallPortrait(): string;
  abstract get fullPortrait(): string;
  abstract get initiative(): number;
  abstract get conditions(): Condition[];

  static create(model: Participant): ParticipantViewModel {
    if ("monster" in model) {
      return new MonsterViewModel(model.monster);
    } else if ("player" in model) {
      return new PlayerViewModel(model.player);
    } else if ("lair" in model) {
      return new LairViewModel(model.lair);
    }
    throw new Error("Unknown participant type");
  }
}

export class MonsterViewModel extends ParticipantViewModel {
  get name(): string {
    return this._model.name;
  }

  get smallPortrait(): string {
    const portrait = this._model.def.portrait || "unknown-monster";
    return `/images/portraits/${portrait}.small.jpg`;
  }

  get fullPortrait(): string {
    const portrait = this._model.def.portrait || "unknown-monster";
    return `/images/portraits/${portrait}.full.jpg`;
  }

  get initiative(): number {
    return this._model.initiative;
  }

  get conditions(): Condition[] {
    return this._model.conditions.slice().map(s => s).sort((a, b) =>
      (conditionPriorities[a.name] || 1000) - (conditionPriorities[b.name] || 1000)
    );
  }

  get hp(): number {
    return this._model.hp;
  }

  get maxHp(): number {
    return this._model.maxHp;
  }

  get tempHp(): number {
    return this._model.tempHp;
  }

  get ac(): number {
    return this._model.def.ac;
  }

  get stats(): Stats {
    return this._model.def.stats;
  }

  get action(): boolean {
    return this._model.action;
  }

  get reaction(): boolean {
    return this._model.reaction;
  }

  get bonusAction(): boolean {
    return this._model.bonusAction;
  }

  get legendaryActions(): number {
    return this._model.legendaryActions;
  }

  get totalLegendaryActions(): number {
    return this._model.def.legendaryActions;
  }

  constructor(private _model: Monster) {
    super();
  }
}

export class PlayerViewModel extends ParticipantViewModel {
  get name(): string {
    return this._model.def.name;
  }

  get smallPortrait(): string {
    const portrait = this._model.def.portrait || "unknown-player";
    return `/images/portraits/${portrait}.small.jpg`;
  }

  get fullPortrait(): string {
    const portrait = this._model.def.portrait || "unknown-player";
    return `/images/portraits/${portrait}.full.jpg`;
  }

  get initiative(): number {
    return this._model.initiative;
  }

  get conditions(): Condition[] {
    return this._model.conditions.slice().map(s => s).sort((a, b) =>
      (conditionPriorities[a.name] ?? 1000) - (conditionPriorities[b.name] ?? 1000)
    );
  }

  constructor(private _model: Player) {
    super();
  }
}

export class LairViewModel extends ParticipantViewModel {
  get name(): string {
    return this._model.name;
  }

  get smallPortrait(): string {
    const portrait = this._model.portrait || "lair";
    return `/images/portraits/${portrait}.small.jpg`;
  }

  get fullPortrait(): string {
    const portrait = this._model.portrait || "lair";
    return `/images/portraits/${portrait}.full.jpg`;
  }

  get conditions(): Condition[] {
    return [];
  }

  get initiative(): number {
    return 20;
  }

  constructor(private _model: Lair) {
    super();
  }
}

const conditionPriorities: Record<Condition["name"], number> = {
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
