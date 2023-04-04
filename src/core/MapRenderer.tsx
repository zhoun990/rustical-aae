import { ReactNode, useEffect, useRef, useState } from "react";
import { Region } from "../../typeshare";
import { mapSource } from "../App";
import { N } from "../utils/NumericUtils";
import { relaunch } from "@tauri-apps/api/process";
import { invoke } from "@tauri-apps/api/tauri";

const STEP = [
	1600, 1300, 1000, 800, 700, 600, 550, 500, 450, 400, 350, 300, 250, 200, 170,
	140, 120, 100,
]; // 最後の値を100%とした時のn%, 100%=full size(height=screenHeight)
const IMAGE_SCALE_INDEX = STEP.length - 1;
const WIDTH = 18000; //Number(M.props.width); /// (1 + IMAGE_SCALE_INDEX * SCALE_STEP);
const HEIGHT = 3000; //Number(M.props.height); /// (1 + IMAGE_SCALE_INDEX * SCALE_STEP);
const exponential = (n: number) => 2 ** ((1 - n) * 10) / 2 ** 10;

export const MapRenderer: React.FC<{
	children?: ReactNode;
	regions: Map<number, Region>;
	timestamp: number;
}> = ({ regions, children,timestamp }) => {
	const ref = useRef<SVGSVGElement>(null);
	const mapRef = useRef({
		w: WIDTH,
		h: HEIGHT,
		x: 0,
		y: 0,
		i: IMAGE_SCALE_INDEX,
	});
	const [mapData, setMapData] = useState(Object.assign({}, mapRef.current));
	const mouseState = useRef({
		t: false,
		r: false,
		b: false,
		l: false,
		wheelDown: false,
		leftDown: false,
	});
	useEffect(() => {
		mapRef.current = Object.assign({}, mapData);
	}, [mapData]);
	useEffect(() => {
		function mapHandler(e: any) {
			const cw = document.documentElement.clientWidth;
			const ch = document.documentElement.clientHeight;
			const scaled = (i: number) => {
				const h = STEP[i] * (ch / STEP[STEP.length - 1]);
				return { w: N.ratio(HEIGHT, h, WIDTH), h };
			};
			let { i, w, h, x, y } = mapRef.current;
			let update = true;
			const dx = (n: number) =>
				Math.abs(Math.abs(mapRef.current.w) - Math.abs(scaled(n).w)) || 0;
			const dy = (n: number) =>
				Math.abs(Math.abs(mapRef.current.h) - Math.abs(scaled(n).h)) || 0;
			if (e.wheelDelta > 0) {
				//Zoom In
				i--;
				if (i < 0) update = false;
				const movement = (type: "x" | "y") => {
					const CORRECTION_INTENSITY = 1.3;
					let offset = e.offsetX,
						size = w,
						clientSize = cw,
						pos = x,
						df = dx(i);

					if (type === "y") {
						offset = e.offsetY;
						size = h;
						clientSize = ch;
						pos = y;
						df = dy(i);
					} //else console.log(e.clientX);
					const d = offset / size - (clientSize / 2 - pos) / size;
					const dd =
						d + d * (exponential(i / (STEP.length - 1)) * CORRECTION_INTENSITY);
					return df * ((clientSize / 2 - pos) / size + dd);
				};
				x -= movement("x");
				y -= movement("y");
			} else {
				i++;
				if (i > STEP.length - 1) update = false;
				x += dx(i) * ((cw / 2 - x) / w);
				y += dy(i) * ((ch / 2 - y) / h);
			}
			if (update)
				setMapData({
					i,
					w: scaled(i).w,
					h: scaled(i).h,
					x,
					y: Math.min(
						0,
						y + scaled(i).h < ch ? y + (ch - (y + scaled(i).h)) : y
					),
				});
		}

		let mouseMoveX: number, mouseMoveY: number, wheelDragX, wheelDragY;
		let zoomLeftBuf = 0,
			zoomTopBuf = 0,
			gameSpeed = 1,
			isSpaceDown = 0,
			isControlDown = false;

		function wheelMove(e: { clientX: number; clientY: number }) {
			if (mouseState.current.wheelDown && mouseMoveX && mouseMoveY) {
				const ch = document.documentElement.clientHeight;
				let { i, w, h, x, y } = mapRef.current;

				// ドラッグ処理
				wheelDragX = e.clientX;
				wheelDragY = e.clientY;

				x = zoomLeftBuf - (mouseMoveX - wheelDragX); // / 1; //imageScale;
				// x = Math.max(0, Math.min(cw - w, x));

				y = zoomTopBuf - (mouseMoveY - wheelDragY); // / 1; //imageScale;
				// y = Math.max(0, Math.min(ch - h, y));

				setMapData({
					i,
					w,
					h,
					x,
					y: Math.min(0, y + h < ch ? y + (ch - (y + h)) : y),
				});
			} else {
				// 移動座標の記録
				mouseMoveX = e.clientX;
				mouseMoveY = e.clientY;
			}
		}
		function mouseMove(e: MouseEvent) {
			if (mouseState.current.leftDown) {
				// console.log(
				// 	mapRef.current.x > 0,
				// 	mapRef.current.x +
				// 		mapRef.current.w -
				// 		document.documentElement.clientWidth <
				// 		0
				// );

				const x = N.ratio(ref.current?.clientWidth || 0, e.offsetX, WIDTH);
				const y = N.ratio(ref.current?.clientHeight || 0, e.offsetY, HEIGHT);
				// setSelectArmies((value) => {
				// 	const array = value.concat();
				// 	let index = 0;
				// 	if (array.length) {
				// 		index = 1;
				// 	} else {
				// 	}
				// 	// console.log(array.length, e.clientX);
				// 	array[index] = {
				// 		x,
				// 		y,
				// 	};
				// 	return array;
				// });
			} else {
				// setSelectArmies([]);
			}
		}
		const keydownEvent = (e: KeyboardEvent) => {
			if (e.key === " " && e.target == document.body) {
				e.preventDefault();
			}
			const movement = 10;
			const m = { x: 0, y: 0 };
			// if (!typingRef.current) {
			switch (e.key) {
				// case "Escape":
				// 	(
				// 		confirm("Exit and go stat screen?") as unknown as Promise<boolean>
				// 	).then((b) => {
				// 		if (b) {
				// 			relaunch();
				// 		}
				// 	});
				// 	break;
				case " ":
					isSpaceDown++;
					break;
				case "Control":
					isControlDown = true;
					break;
				case "w":
					console.log("w");
					m.y += 10;
					break;
				case "a":
					console.log("a");
					m.x += 10;
					break;
				case "s":
					console.log("s");
					m.y -= 10;
					break;
				case "d":
					console.log("d");
					m.x -= 10;
					break;

				default:
					break;
			}
			if (m.y || m.x)
				setMapData((c) => ({
					...c,
					x: mapRef.current.x + movement * m.x,
					y: mapRef.current.y + movement * m.y,
				}));
			// if (e.key === "Escape") {
			// 	dispatch(
			// 		actions.set({
			// 			modal: "menu",
			// 		})
			// 	);
			// }
			// if (e.key === " ") {
			// 	isSpaceDown++;
			// }
			// if (e.key === "Control") {
			// 	isControlDown = true;
			// }
			if (
				e.key === "1" ||
				e.key === "2" ||
				e.key === "3" ||
				e.key === "4" ||
				e.key === "5"
			) {
				gameSpeed = Number(e.key) * (isControlDown ? 10 : 1);
				// //dispatch(actions.set({ play: (value) => value && gameSpeed }));
				// GameManager.set("play", gameSpeed);
			}

			if (isSpaceDown === 1) {
				// // dispatch(
				// // 	actions.set({ play: (value) => (value === 0 ? gameSpeed : 0) })
				// // );
				// GameManager.set("play", (value) => (value === 0 ? gameSpeed : 0));
			}
			// }
		};
		const keyupEvent = (e: KeyboardEvent) => {
			// if (!typingRef.current) {
			if (e.key === " ") {
				isSpaceDown = 0;
			}
			if (e.key === "Control") {
				isControlDown = false;
			}
			// }
		};
		ref.current?.addEventListener("mousewheel", mapHandler);

		// ドラッグ操作用
		ref.current?.addEventListener("mousedown", (e) => {
			if (e.button === 0) {
				// マウスが押下された瞬間の情報を記録
				zoomLeftBuf = mapRef.current.x;
				zoomTopBuf = mapRef.current.y;
				mouseState.current.leftDown = true;
			}
			if (e.button === 1) {
				// マウスが押下された瞬間の情報を記録
				zoomLeftBuf = mapRef.current.x;
				zoomTopBuf = mapRef.current.y;
				mouseState.current.wheelDown = true;
			}
		});
		ref.current?.addEventListener("mouseup", (e) => {
			if (e.button === 0) mouseState.current.leftDown = false;
			if (e.button === 1) mouseState.current.wheelDown = false;
		});
		ref.current?.addEventListener("mouseout", function (e) {
			if (
				ref.current?.contains(e.relatedTarget as any) ||
				e.relatedTarget === null
			)
				return;
			mouseState.current.wheelDown = false;
		});
		ref.current?.addEventListener("mousemove", wheelMove);
		ref.current?.addEventListener("mousemove", mouseMove);
		document.addEventListener("keydown", keydownEvent);
		document.addEventListener("keyup", keyupEvent);
	}, []);
	const hoverListener = (type: "t" | "r" | "b" | "l") => {
		return {
			onMouseOver: () => (mouseState.current[type] = true),
			onMouseLeave: () => (mouseState.current[type] = false),
		};
	};
	const moveMapPos = () => {
		let mx = 0,
			my = 0;
		if (mouseState.current.t || mouseState.current.b) {
			my = mouseState.current.t ? 20 : -20;
		} else if (mouseState.current.l || mouseState.current.r) {
			mx = mouseState.current.l ? 20 : -20;
		}
		const y = mapRef.current.y + my;
		const ch = document.documentElement.clientHeight;
		(mx || my) &&
			!mouseState.current.wheelDown &&
			setMapData({
				...mapRef.current,
				x: mapRef.current.x + mx,
				y: Math.min(
					0,
					y + mapRef.current.h < ch ? y + (ch - (y + mapRef.current.h)) : y
				),
			});
	};
	useEffect(() => {
		const t = setInterval(moveMapPos, 50);

		return () => {
			clearInterval(t);
			console.log("^_^ Log \n file: index.tsx \n line 159 \n clearInterval");
		};
	}, []);
	return (
		<div className="overflow-hidden h-screen w-screen bg-blue-500">
			<svg
				// width={mapData.w}
				height={mapData.h}
				viewBox="0 0 18000 3000"
				// viewBox={`0 0 ${WIDTH * 1.5} ${HEIGHT * 1.5}`}
				// ref={ref}
				className="fixed flex-none border"
				style={{ left: mapData.x, top: mapData.y }}
				ref={ref}
				// onClick={() => {
				// 	GameManager.set("focus", { type: null, id: "" });
				// }}
			>
				{Array.from(regions.values())
					.sort((a, b) => a.id - b.id)
					.map((region, i) => {
						const map = mapSource.get(region.id);
						return (
							<path
								key={i}
								d={map?.d}
								fill={map?.fill}
								stroke={map?.stroke}
							></path>
						);
					})}
			</svg>
			<div
				className="fixed w-screen h-[20px] top-0 left-0 z-40 text-center"
				{...hoverListener("t")}
			/>
			<div
				className="fixed w-[20px] h-screen top-0 right-0 z-50"
				{...hoverListener("r")}
			/>
			<div
				className="fixed w-screen h-[20px] bottom-0 left-0 z-40"
				{...hoverListener("b")}
			/>
			<div
				className="fixed w-[20px] h-screen top-0 left-0 z-50"
				{...hoverListener("l")}
			/>
			{children}
		</div>
	);
};
