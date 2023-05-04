import { client } from "../../client";
import { store } from "../../main";
import { Citizen } from "../../types/rspc/bindings";
import { GameCache } from "../../utils/GameCache";
import { YEAR } from "../../utils/constans";
import { RegionClass } from "./Region";

export class CitizenClass {
	constructor(public id: number) {}
	get self() {
		return store.state.citizens.get(this.id)!;
	}
	update(v: Citizen | ((current: Citizen) => Citizen)) {
		const value = typeof v === "function" ? v(this.self) : v;
		client.query(["app.updateCitizen", value]);
		store.set({
			citizens: store.state.citizens.set(this.id, value),
		});
	}
	get age() {
		return Math.floor(
			(store.state.timestamp - this.self.born_timestamp) / YEAR
		);
	}
}
