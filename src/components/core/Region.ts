import { client } from "../../client";
import { store } from "../../main";
import { Region } from "../../types/rspc/bindings";
import { GameCache } from "../../utils/GameCache";
import { N } from "../../utils/NumericUtils";
import { CityClass } from "./City";

export class RegionClass {
	constructor(public id: number) {}
	get self() {
		return store.state.regions.get(this.id)!;
	}
	update(v: Region | ((current: Region) => Region)) {
		const value = typeof v === "function" ? v(this.self) : v;
		client.query(["app.updateRegion", value]);
		store.set({
			regions: store.state.regions.set(this.id, value),
		});
	}
	get cities() {
		return Array.from(store.state.cities.values())
			.filter((v) => v.region_id === this.id)
			.map((v) => new CityClass(v.id));
	}

	get environment() {
		let settlements = this.cities;
		return N.round(
			settlements.reduce((acc, city) => city.self.environment, 0) /
				settlements.length,
			3
		);
	}

	get dev() {
		return GameCache.use("region-dev" + this.self.id, () =>
			this.cities.reduce(
				(acc, city) => {
					(["building", "production", "infrastructure"] as const).forEach(
						(type) => {
							acc[type] += city.self[`dev_${type}`];
						}
					);
					return acc;
				},
				{ production: 0, building: 0, infrastructure: 0 }
			)
		);
	}
}
