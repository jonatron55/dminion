// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

import { diceCommands, type diceCommands as DiceCommands } from "$lib/model/Commands";

export type HistoryItem = {
  expression: string;
  roll?: DiceCommands.Roll;
  error?: string;
};

export class DiceViewModel {
  public history: HistoryItem[] = [];

  async roll(expr: string): Promise<DiceCommands.Roll> {
    try {
      const roll = await diceCommands.roll({ expr });
      this.history = [...this.history, { expression: expr, roll }];
      return roll;
    } catch (e) {
      this.history = [...this.history, { expression: expr, error: `${e}` }];
      throw e;
    }
  }

  clearHistory(): void {
    this.history = [];
  }
}
