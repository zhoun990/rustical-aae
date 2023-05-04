import { client } from "../../client";
import { store } from "../../main";
import { City } from "../../types/rspc/bindings";
import { GameCache } from "../../utils/GameCache";
import { CitizenClass } from "./Citizren";
import { RegionClass } from "./Region";

export class CityClass {
	constructor(public id: number) {}
	get self() {
		return store.state.cities.get(this.id)!;
	}
	update(v: City | ((current: City) => City)) {
		const value = typeof v === "function" ? v(this.self) : v;
		client.query(["app.updateCity", value]);
		store.set({
			cities: store.state.cities.set(this.id, value),
		});
	}
	get region() {
		return new RegionClass(this.self.region_id);
	}
	get citizens() {
		return Array.from(store.state.citizens.values())
			.filter((v) => v.staying_city_id === this.id)
			.map((v) => new CitizenClass(v.id));
	}
}
