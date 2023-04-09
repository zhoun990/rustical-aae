import { FC, useContext } from "react";
import { Store } from "../../store";
// import { CountryPanel } from "./CountryPanel";
import { StatePanel } from "./StatePanel";

const isProd: boolean = process.env.NODE_ENV === "production";

export const InfoPanel: FC<{ editable?: boolean }> = ({ editable }) => {
	const { focus } = useContext(Store);
	if (!focus) return null;
	if (focus.type === "state") return <StatePanel editable={editable} />;
	// if (focus.type === "country") return <CountryPanel editable={editable} />;
	return null;
};
