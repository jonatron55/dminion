// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

import { messageBoxStore } from "$lib/MessageBox";
import { invoke, type InvokeArgs } from "@tauri-apps/api/core";
import type { Condition } from "./Condition";
import type { Damage, Healing } from "./Damage";
import type { Game } from "./Game";
import type { Action } from "./Participant";

export namespace gameCommands {
  export interface DamageArgs {
    [key: string]: unknown;
    target: number,
    damage: Damage,
  }

  export interface HealArgs {
    [key: string]: unknown;
    target: number,
    healing: Healing
  }

  export interface SetActionArgs {
    [key: string]: unknown;
    target: number,
    action: Action,
    available: boolean
  }

  export interface AddConditionsArgs {
    [key: string]: unknown;
    target: number;
    conditions: Condition[];
  }

  export const newGame = async (): Promise<void> => await tryInvoke("new_game");
  export const getGame = async (): Promise<Game> => await tryInvoke("get_game");
  export const nextTurn = async (): Promise<void> => await tryInvoke("next_turn");
  export const undo = async (): Promise<void> => await tryInvoke("undo");
  export const redo = async (): Promise<void> => await tryInvoke("redo");
  export const damage = async (args: DamageArgs): Promise<void> => await tryInvoke("damage", args);
  export const heal = async (args: HealArgs): Promise<void> => await tryInvoke("heal", args);
  export const setAction = async (args: SetActionArgs): Promise<void> => await tryInvoke("set_action", args);
  export const addConditions = async (args: AddConditionsArgs): Promise<void> =>
    await tryInvoke("add_conditions", args);
}

export namespace diceCommands {
  export interface DieRoll {
    sides: number;
    result: number;
    keep: boolean;
  }

  export interface Roll {
    value: number;
    dice: DieRoll[];
  }

  export interface RollArgs {
    [key: string]: unknown;
    expr: string;
  }

  export const roll = async (args: RollArgs): Promise<Roll> => await tryInvoke("roll", args);
}

async function tryInvoke(command: string, args?: InvokeArgs): Promise<any> {
  try {
    console.trace(`Invoke: ${command}(${JSON.stringify(args, null, 2)})`);
    const result = await invoke(command, args);
    console.trace(` ${command} result: ${JSON.stringify(result, null, 2)}`);
    return result;
  } catch (e) {
    messageBoxStore.show({
      title: `Failed to execute ${command}`,
      content: `${e}`,
      severity: "danger",
      affirmativeButton: { label: "OK" }
    });

    throw e;
  }
}
