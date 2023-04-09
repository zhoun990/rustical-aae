import { createContext, Dispatch, Reducer, ReducerAction } from "react";
import { initialState } from "./InitialState";
import { createSlice } from "./createSlice";
import { clone } from "../utils/rfdc";
import { InitialState } from "./InitialState";

const slice = createSlice({
	initialState,
	reducers: {
		set: <
			T extends keyof InitialState,
			Payload extends {
				[key in T]?:
					| InitialState[key]
					| ((value: InitialState[key]) => InitialState[key]);
			}
		>(
			state: any,
			payload: Payload
		) => {
			(Object.keys(payload) as Array<keyof Payload>).forEach((key) => {
				const cb = payload[key];
				if (typeof cb == "function") {
					try {
						state[key] = cb(clone(state[key]));
					} catch (err) {
						console.error({ key, value: state[key], err });
					}
				} else {
					state[key] = payload[key];
				}
			});
		},
	},
});

export const { reducer, actions } = slice;
export default slice;
export const Store = createContext({
	...initialState,
	dispatch: undefined,
	set: undefined,
} as unknown as InitialState & { dispatch: Dispatch<ReducerAction<Reducer<any, any>>>; set: <T extends keyof InitialState>(value: { [key in T]?: InitialState[key] | ((value: InitialState[key]) => InitialState[key]) | undefined }) => void });
