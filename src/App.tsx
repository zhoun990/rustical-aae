import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
// import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";

import "./App.css";
import { Tooltip } from "./components/Interface/Tooltip";
import { Citizen, City, Region } from "../typeshare";
import { fs, path } from "@tauri-apps/api";
import { MapRenderer } from "./core/MapRenderer";
import { relaunch } from "@tauri-apps/api/process";
import { date } from "./utils/date";
import { MapData } from "./types/MapData";
import { genMapData } from "./utils/genMapData";
import { SavedData } from "./components/SavedData";
import { client } from "./client";

export let mapSource: MapData = new Map();
function App() {
	const [pause, setPause] = useState(true);
	const [speed, setSpeed] = useState(1);
	const [gameId, setGameId] = useState("");
	// const [mapData, mapData=] = useState<MapData>(new Map());
	const [citizens, setCitizens] = useState(new Map<number, Citizen>());
	const [cities, setCities] = useState(new Map<number, City>());
	const [regions, setRegions] = useState(new Map<number, Region>());
	const [timestamp, setTimestamp] = useState(0);

	const handlePause = (bool: boolean) => {
		console.log("^_^ Log \n file: App.tsx:140 \n bool:", bool);
		setPause(bool);
		handleSpeed(bool ? 0 : speed);
	};
	async function handleSpeed(n: number) {
		console.log("^_^ Log \n file: App.tsx:145 \n n:", n);
		// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
		// await invoke("play_speed_update", { speed: n });
		client.query(["app.playSpeedUpdate", n]).then((res) => {
			console.log(res);
		});
		if (n) {
			setPause(false);
			setSpeed(n);
		}
	}
	useEffect(() => {
		console.log({ a: regions, b: mapSource });
	}, [regions]);
	useEffect(() => {
		genMapData().then((map) => {
			mapSource = map;
			// invoke("refresh");
			client.query(["app.refresh"]);
		});
		listen("set_game_id", ({ event, payload }) => {
			setGameId(payload as string);
		});
		listen(
			"game_data",
			({
				payload,
			}: {
				payload: {
					game_id: string;
					citizens: [number, Citizen][];
					cities: [number, City][];
					regions: [number, Region][];
					timestamp: number;
				};
			}) => {
				setCitizens(new Map(payload.citizens));
				setCities(new Map(payload.cities));
				setRegions(new Map(payload.regions));
				setTimestamp(payload.timestamp);
			}
		);
		// MAP_DATA.forEach((value, key) => {});
		const handleKey = (event: KeyboardEvent) => {
			switch (event.key) {
				case "Escape":
					console.log("^_^ Log \n file: App.tsx:189 \n Escape:");
					(
						confirm("Exit and go stat screen?") as unknown as Promise<boolean>
					).then((b) => {
						if (b) {
							relaunch();
						}
					});
					break;
				case " ":
					console.log("^_^ Log \n file: App.tsx:189 \n Escape:");
					setPause((c) => {
						handleSpeed(!c ? 0 : speed);
						return !c;
					});
					break;
			}
			if (
				event.key === "1" ||
				event.key === "2" ||
				event.key === "3" ||
				event.key === "4" ||
				event.key === "5"
			) {
				handleSpeed(Number(event.key));
			}
		};
		window.addEventListener("keydown", (e) => {
			// if (timeoutId) return;

			// timeoutId = setTimeout(function () {
			//   timeoutId = 0;
			//   // 処理内容
			handleKey(e);
			// }, 100);
		});
		return () => {
			// if (timeoutId) clearTimeout(timeoutId);
			window.removeEventListener("keydown", handleKey);
		};
	}, []);
	if (gameId)
		return (
			<MapRenderer regions={regions} timestamp={timestamp}>
				<div className="fixed top-10 left-10">
					<h1>AAE</h1>
					<div className="fixed top-0 right-0 text-lg text-white bg-gray-400 p-3">
						<Tooltip content="time tool tip daaaaaaaaaaaaaa log message a log message a log message a log message a log message a log message a log message a log message">
							time:{date(timestamp).full}
						</Tooltip>
						{/* <Tooltip content="this is log">Log</Tooltip> */}

						<div>speed:{speed}</div>
					</div>
					<p>Speed:{speed}</p>
					<div className="row">
						<button onClick={() => handleSpeed(1)}>1</button>
						<button onClick={() => handleSpeed(2)}>2</button>
						<button onClick={() => handleSpeed(3)}>3</button>
						<button onClick={() => handleSpeed(4)}>4</button>
						<button onClick={() => handleSpeed(5)}>5</button>
					</div>
					<div className="row">
						<button
							className="p-1 border hover:opacity-75"
							onClick={() => {
								console.log("onclick");

								handlePause(!pause);
							}}
						>
							{pause ? "Start" : "Stop"}
						</button>
					</div>
					<div>{mapSource.size}</div>
				</div>
			</MapRenderer>
		);
	return (
		<div className="flex items-center justify-center h-full flex-col">
			<div
				className="m-2 p-2 border rounded-lg text-2xl mt-10"
				onClick={() => {
					// invoke("select_game_id", { gameId: null });
					client.query(["app.selectGameId", null]);
				}}
			>
				New Game
			</div>
			<div
				className="m-2 p-2 border rounded-lg text-2xl"
				onClick={() => {
					const map: Record<number, Region> = {};
					mapSource.forEach((v, k) => {
						const r: Region = {
							id: k,
							name: "",
							product: "",
							position_x: v.position.x,
							position_y: v.position.y,
						};
						map[k] = r;
					});
					// invoke("init_game", {
					// 	regions: map,
					// });
					// client.query(["app.initGame", map]);

				}}
			>
				Init and New Game
			</div>
			<SavedData />
		</div>
	);
}

export default App;
