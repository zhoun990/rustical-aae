import React, { FC, ReactNode, useReducer } from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";
import { Store, actions, reducer } from "./store";
import { InitialState, initialState } from "./store/InitialState";
const StoreProvider: FC<{ children: ReactNode }> = ({ children }) => {
	const [state, dispatch] = useReducer(reducer, initialState);
	const set = <T extends keyof InitialState>(value: {
		[key in T]?:
			| InitialState[key]
			| ((value: InitialState[key]) => InitialState[key]);
	}) => {
		dispatch(actions.set(value));
	};
	return (
		<Store.Provider value={{ ...state, dispatch, set }}>
			{children}
		</Store.Provider>
	);
};

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
	<React.StrictMode>
		<StoreProvider>
			<App />
		</StoreProvider>
	</React.StrictMode>
);
