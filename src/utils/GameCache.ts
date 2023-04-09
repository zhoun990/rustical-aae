import { store } from "../main";

export class GameCache {
	cache: Map<string, { time: number; clockTime: number; value: any }> =
		new Map();
	private constructor() {}
	private static instance: GameCache;
	private static getInstance(): GameCache {
		if (!GameCache.instance) {
			GameCache.instance = new GameCache();
		}
		return GameCache.instance;
	}
	static use<Value>(
		cacheKey: string,
		callback: () => Value,
		timeSpan = 100
	): Value {
		const cache = this.getInstance().cache;
		const cachedValue = cache.get(cacheKey);
		const time = store.state.timestamp;
		const clockTime = Math.floor(Date.now() / timeSpan);
		if (
			cachedValue &&
			cachedValue.time === time &&
			cachedValue.clockTime === clockTime
		) {
			return cachedValue.value;
		}
		const newValue = callback();
		cache.set(cacheKey, { time, clockTime, value: newValue });
		return newValue;
	}
}
// const cache: Map<string, { time: number; clockTime: number; value: any }> =
// 	new Map();
// export const GameCache = {
// 	use<Value>(cacheKey: string, callback: () => Value, timeSpan = 100): Value {
// 		const cachedValue = cache.get(cacheKey);
// 		const time = GameManager.store.time;
// 		const clockTime = Math.floor(Date.now() / timeSpan);
// 		if (
// 			cachedValue &&
// 			cachedValue.time === time &&
// 			cachedValue.clockTime === clockTime
// 		) {
// 			return cachedValue.value;
// 		}
// 		const newValue = callback();
// 		cache.set(cacheKey, { time, clockTime, value: newValue });
// 		return newValue;
// 	},
// };
