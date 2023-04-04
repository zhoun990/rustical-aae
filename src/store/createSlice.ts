type Reducers<ContextState> = (
	state: ContextState,
	payload: any
) => void | ContextState;
type Action<ContextState> = (state: ContextState) => void | ContextState;
type Reducer<ContextState> = (
	state: ContextState,
	action: Action<ContextState>
) => ContextState;
type Actions<ContextState, Payload> = (
	payload: Payload
) => Action<ContextState>;
type ArgumentTypes<F extends Function> = F extends (...args: infer A) => any
	? A
	: never;

export const createSlice = <
	InitialState extends { debug: boolean },
	R extends Record<string, Reducers<InitialState>>
>({
	initialState,
	reducers,
}: {
	initialState: InitialState;
	reducers: R;
}) => {
	type isNecessary<T> = T extends undefined ? [] : [T];
	type RA1<F extends Function> = ArgumentTypes<F>[1];
	const actions = {} as {
		[key in keyof R]: (
			...payload: isNecessary<RA1<R[key]>>
		) => Action<InitialState>;
	};
	const reducer: Reducer<InitialState> = (state, action) => {
		let s1 = { debug: false };
		// if (state["debug"]) {
		// 	s1 = clone(state);
		// }
		// const s1 = JSON.parse(JSON.stringify(state));
		const s = action(state);
		if (s) {
			state = s;
		}
		// if (s1?.["debug"]) {
		// 	const s2 = clone(state);
		// 	const log: any = {};
		// 	getObjectKeys(s1).forEach((key) => {
		// 		if (JSON.stringify(s1[key]) !== JSON.stringify(s2[key]))
		// 			log[key] = s2[key];
		// 	});
		// 	Object.keys(log).length && console.log(log);
		// }

		return Object.assign({}, state);
	};
	(Object.keys(reducers) as Array<keyof R>).forEach((key) => {
		actions[key] =
			(...payload) =>
			(state: InitialState) =>
				reducers[key](state, payload[0]);
		// const a =
		// 	(...payload) =>
		// 	(state: InitialState) =>
		// 		reducers[key](state, payload[0]);
	});
	return {
		actions,
		reducer,
		initialState,
	};
};
