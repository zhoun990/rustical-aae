import { YEAR, MONTH } from "./constans";

export const date = (time: number) => {
	const day = time / 24;
	const hour = time % 24;
	const y = Math.floor(day / YEAR);
	const m = Math.floor((day - YEAR * y) / MONTH);
	const d = Math.floor(day - YEAR * y - MONTH * m) + 1;
	return {
		full: `${y + 1}/${m + 1}/${d}/${hour}`,
		day: d,
		month: m + 1,
		year: y + 1,
		hour: hour,
	};
};
