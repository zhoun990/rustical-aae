import { invoke } from "@tauri-apps/api/tauri";
import { PlaySpeedUpdate } from "../../typeshare";

export const playSpeedUpdate = async (args: PlaySpeedUpdate) =>
	await invoke("play_speed_update", { ...args });
