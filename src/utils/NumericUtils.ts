/**
 * 2つの数値を指定された演算子で比較します。
 * @param n1 最初の数値
 * @param operator 比較に使用する演算子
 * @param n2 2番目の数値
 * @returns 比較結果（真偽値）
 */
const c = (
	a: number,
	operator: ">" | "<" | ">=" | "<=",
	b: number
): boolean => {
	switch (operator) {
		case "<": {
			return a < b;
		}
		case ">": {
			return a > b;
		}
		case "<=": {
			return a <= b;
		}
		case ">=": {
			return a >= b;
		}
	}
};
export class N {
	/**
	 * 指定された演算子を使用して、2つまたは3つの数値を比較します。
	 * @param n1 最初の数値
	 * @param operator1 最初の数値と2番目の数値の間で使用する演算子
	 * @param n2 2番目の数値
	 * @param operator2 2番目の数値と3番目の数値の間で使用する演算子
	 * @param n3 3番目の数値
	 * @returns 比較結果（真偽値）
	 */
	static compare(
		n1: number,
		operator1: "<" | "<=",
		n2: number,
		operator2?: "<" | "<=",
		n3?: number
	): boolean {
		if (operator2 && n3 !== undefined)
			return c(n1, operator1, n2) && c(n2, operator1, n3);
		else return c(n1, operator1, n2);
	}
	/**
	 * 指定された桁数で数値を四捨五入する
	 * @param value 丸めたい数値
	 * @param precision 丸める桁数
	 * @returns 丸められた数値
	 */
	static round(value: number, precision = 1) {
		const multiplier = Math.pow(10, precision);
		return Math.round(value * multiplier) / multiplier || 0;
	}
	/**
	 * 指定された桁数で数値を書式化する。
	 * 1,000以上の場合は1,000あたり"1k"、
	 * 1,000,000以上の場合は1,000,000あたり"1M"として表示する。
	 * @param num 書式化する数値
	 * @returns 書式化された文字列
	 */
	static formatNumber(num: number, round = 1): string {
		if (num >= 100000) {
			return (
				N.round(num / 1000000, round).toLocaleString("en-US", {
					minimumFractionDigits: 0,
				}) + "M"
			);
		} else if (num >= 100) {
			return (
				N.round(num / 1000, round).toLocaleString("en-US", {
					minimumFractionDigits: 0,
				}) + "K"
			);
		} else {
			return N.round(num, round).toLocaleString("en-US");
		}
	}
	/**
	 *return a number(min <= n <= max)
	 */
	static random(min: number, max: number) {
		max++;
		return Math.floor(Math.random() * (Math.max(max, min) - min) + min);
	}
	/** listのlengthが0の場合、undefinedを返す。
	 */
	static pick<T>(list: Array<T>) {
		// if (!list) return undefined;
		return list[N.random(0, list.length - 1)];
	}
	/**
	 *
	 * @params a : b = c : x
	 * @returns x
	 */
	static ratio(a: number, b: number, c: number): number {
		return (b * c) / a;
	}
}
