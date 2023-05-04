import { Citizen, City, Country, Region } from "../types/rspc/bindings";

const isProd: boolean = process.env.NODE_ENV === "production";
export interface InitialState {
	debug: boolean;
	play: number;
	modal: "load" | "menu" | "createGod" | "load" | "save" | "";
	focus: { type: "state" | "country" | null; id: number | null };
	countries: Map<number, Country>;
	regions: Map<number, Region>;
	cities: Map<number, City>;
	citizens: Map<number, Citizen>;
	timestamp: number;
	mode: "default" | "battle";
}
export const initialState: InitialState = {
	debug: false,
	play: 0,
	modal: isProd ? "load" : "",
	focus: { type: null, id: null },
	countries: new Map(),
	regions: new Map(),
	cities: new Map(),
	citizens: new Map(),
	timestamp: 0,
	mode: "default",
};
