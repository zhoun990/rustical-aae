import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
	const [pause, setPause] = useState(true);
	const [speed, setSpeed] = useState(1);
	const handlePause = (bool: boolean) => {
		setPause(bool);
		handleSpeed(bool ? 0 : speed);
	};
	async function handleSpeed(n: number) {
		if (n) setPause(false);
		// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
		await invoke("play_speed_update", { speed: n });
		setSpeed(n);
	}

	return (
		<div className="container">
			<h1>Welcome to Tauri!</h1>

			<div className="row">
				<a href="https://vitejs.dev" target="_blank">
					<img src="/vite.svg" className="logo vite" alt="Vite logo" />
				</a>
				<a href="https://tauri.app" target="_blank">
					<img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
				</a>
				<a href="https://reactjs.org" target="_blank">
					<img src={reactLogo} className="logo react" alt="React logo" />
				</a>
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
				<button onClick={() => handlePause(!pause)}>
					{pause ? "Start" : "Stop"}
				</button>
			</div>
		</div>
	);
}

export default App;
