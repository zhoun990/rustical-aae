const isProd: boolean = process.env.NODE_ENV === "production";
export interface InitialState {
    debug: boolean;
    play: number;
    modal: "load" | "menu" | "createGod" | "load" | "save" | "";

}
export const initialState: InitialState = {
	debug: false,
	play: 0,
	modal: isProd ? "load" : "",
};
