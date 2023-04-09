import React, { FC, ReactNode, useEffect, useReducer } from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";
import { Store, actions, reducer } from "./store";
import { InitialState, initialState } from "./store/InitialState";
import { RSPCProvider, client, queryClient } from "./client";
export let store = {
	state: initialState,
	set: (() => {}) as <T extends keyof InitialState>(value: {
		[key in T]?:
			| InitialState[key]
			| ((value: InitialState[key]) => InitialState[key])
			| undefined;
	}) => void,
};
const StoreProvider: FC<{ children: ReactNode }> = ({ children }) => {
	const [state, dispatch] = useReducer(reducer, initialState);
	const set = <T extends keyof InitialState>(value: {
		[key in T]?:
			| InitialState[key]
			| ((value: InitialState[key]) => InitialState[key]);
	}) => {
		dispatch(actions.set(value));
	};
	useEffect(() => {
		store = { state, set };
	}, [state]);

	return (
		<Store.Provider value={{ ...state, dispatch, set }}>
			{children}
		</Store.Provider>
	);
};

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
	<RSPCProvider client={client} queryClient={queryClient}>
		<StoreProvider>
			{/* <React.StrictMode> */}
			<App />
			{/* </React.StrictMode> */}
		</StoreProvider>
	</RSPCProvider>
);
