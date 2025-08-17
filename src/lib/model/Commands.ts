import { invoke } from "@tauri-apps/api/core";
import type { Game } from "./Game";

export namespace game_commands {
  export const newGame = async () => await invoke("new_game") as void;
  export const getGame = async () => await invoke("get_game") as Game;
  export const nextTurn = async () => await invoke("next_turn") as void;
  export const undo = async () => await invoke("undo") as void;
  export const redo = async () => await invoke("redo") as void;
}
