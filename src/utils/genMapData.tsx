import { MapData } from "../types/MapData";

export const genMapData = async (): Promise<MapData> => {
	const map: MapData = new Map();
	const svgFileData = await fetch("map.svg").then((response) =>
		response.text()
	);
	const parser = new DOMParser();
	const svgXml = parser.parseFromString(svgFileData, "image/svg+xml");
	const svgElement = svgXml.getElementsByTagName("svg")[0];

	for (let i = 0; i < svgElement.children.length; i++) {
		const child = svgElement.children[i] as SVGPathElement;
		const a = String(child.getAttribute("d"))
			.split(/[ ]/)
			.filter((value) => value.length > 1)
			.map((value) => value.split(/[,]/).map((n) => Number(n)))
			.filter((value, i) => !(i % 3));
		const x = a.map((value) => value[0]);
		const y = a.map((value) => value[1]);
		let position = { x: 0, y: 0 };

		if (
			!String(child.getAttribute("d"))
				.split(/[ ,]/)
				.filter((value) => value.match(/[^cmz0-9e\.-]/)).length
		) {
			let sum = 0;
			position.x = Math.round(
				x
					.map((value, i) => {
						sum = value + sum;
						return sum;
					})
					.reduce((sum, el) => sum + el, 0) / x.length
			);
			sum = 0;
			position.y = Math.round(
				y
					.map((value, i) => {
						sum = value + sum;
						return sum;
					})
					.reduce((sum, el) => sum + el, 0) / y.length
			);
			map.set(Number(child.id), {
				position,
				id: Number(child.id),
				d: child.getAttribute("d")!,
				fill: child.getAttribute("fill")!,
				stroke: child.getAttribute("stroke")!,
			});
		}
	}
	return map;
};
