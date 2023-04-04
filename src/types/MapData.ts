export type MapData = Map<
	number,
	{
		position: {
			x: number;
			y: number;
		};
		id: number;
		d: string;
		fill: string;
		stroke: string;
	}
>;
