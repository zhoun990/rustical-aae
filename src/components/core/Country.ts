import { client } from "../../client";
import { store } from "../../main";
import { Country } from "../../types/rspc/bindings";
import { GameCache } from "../../utils/GameCache";
import { YEAR } from "../../utils/constans";
import { RegionClass } from "./Region";

export class CountryClass {
	constructor(public id: number) {}
	get self() {
		return store.state.countries.get(this.id)!;
	}
	update(v: Country | ((current: Country) => Country)) {
		const value = typeof v === "function" ? v(this.self) : v;
		client.query(["app.updateCountry", value]);
		store.set({
			countries: store.state.countries.set(this.id, value),
		});
	}
}
