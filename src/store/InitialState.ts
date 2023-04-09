import { Citizen, City, Region } from "../types/rspc/bindings";

const isProd: boolean = process.env.NODE_ENV === "production";
export interface InitialState {
	debug: boolean;
	play: number;
	modal: "load" | "menu" | "createGod" | "load" | "save" | "";
	focus: { type: "state" | "country" | null; id: number | null };
	regions: Map<number, Region>;
	cities: Map<number, City>;
	citizens: Map<number, Citizen>;
	timestamp: number;
}
export const initialState: InitialState = {
	debug: false,
	play: 0,
	modal: isProd ? "load" : "",
	focus: { type: null, id: null },
	regions: new Map(),
	cities: new Map(),
	citizens: new Map(),
	timestamp: 0,
};
