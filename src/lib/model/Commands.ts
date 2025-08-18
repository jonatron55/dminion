import { messageBoxStore } from "$lib/MessageBox";
import { invoke } from "@tauri-apps/api/core";
import type { Game } from "./Game";

export namespace game_commands {
  async function tryInvoke(command: string): Promise<any> {
    try {
      return await invoke(command);
    } catch (e) {
      messageBoxStore.show({
        title: `Failed to execute ${command}`,
        content: `${e}`,
        severity: "danger",
        affirmativeButton: { label: "OK" }
      });

      // throw e;
    }
  }

  export const newGame = async () => await tryInvoke("new_game") as void;
  export const getGame = async () => await tryInvoke("get_game") as Game;
  export const nextTurn = async () => await tryInvoke("next_turn") as void;
  export const undo = async () => await tryInvoke("undo") as void;
  export const redo = async () => await tryInvoke("redo") as void;
}
