import { messageBoxStore } from "$lib/MessageBox";
import { invoke, type InvokeArgs } from "@tauri-apps/api/core";
import type { Damage, Healing } from "./Damage";
import type { Game } from "./Game";
import type { Action, } from "./Participant";

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

  export const newGame = async (): Promise<void> => await tryInvoke("new_game");
  export const getGame = async (): Promise<Game> => await tryInvoke("get_game");
  export const nextTurn = async (): Promise<void> => await tryInvoke("next_turn");
  export const undo = async (): Promise<void> => await tryInvoke("undo");
  export const redo = async (): Promise<void> => await tryInvoke("redo");
  export const damage = async (args: DamageArgs): Promise<void> => await tryInvoke("damage", args);
  export const heal = async (args: HealArgs): Promise<void> => await tryInvoke("heal", args);
  export const setAction = async (args: SetActionArgs): Promise<void> => await tryInvoke("set_action", args);

  async function tryInvoke(command: string, args?: InvokeArgs): Promise<any> {
    try {
      return await invoke(command, args);
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
}
