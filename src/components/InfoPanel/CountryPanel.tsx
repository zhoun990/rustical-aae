import { Tab } from "@headlessui/react";
import { FC, useContext } from "react";
import { actions, Store } from "../../store";
import { CountryClass } from "../core/Country";
import { classNames } from "../../utils/classNames";

const isProd: boolean = process.env.NODE_ENV === "production";
export const CountryPanel: FC<{ editable?: boolean }> = ({ editable }) => {
	const { focus, citizens, countries, set } = useContext(Store);
	const focusId = focus.id;
	const country = (() => {
		if (focusId) {
			const r = countries.get(focusId);
			if (r) return new CountryClass(r.id);
		}
		return undefined;
	})();
	if (!country || !focusId) {
		return null;
	}
	return (
		<div className="fixed w-1/3 top-3 left-3 bg-gray-200 text-lg p-2">
			<div>
				{!isProd && `[${country.id}]`}
				<input
					type="text"
					value={country.self.name}
					onChange={(e) => {
						//[UP]
						// GameManager.set("countries", (value) => {
						// 	value[focusId].name = e.target.value;
						// 	return value;
						// });
					}}
				/>
				<Tab.Group
					manual
					defaultIndex={localStorage.StatePanelTabIndex}
					onChange={(index) => (localStorage.StatePanelTabIndex = index)}
				>
					<Tab.List
						className="flex space-x-1 rounded-xl bg-blue-900/20 p-1"
						onFocus={(e: any) => e.target.blur()}
					>
						{["Government"].map((category) => (
							<Tab
								key={category}
								className={({ selected }) =>
									classNames(
										"w-full rounded-lg py-2.5 text-sm font-medium leading-5 text-blue-700",
										"ring-white ring-opacity-60 ring-offset-2 ring-offset-blue-400 focusId:outline-none focusId:ring-2",
										selected
											? "bg-white shadow"
											: "text-blue-100 hover:bg-white/[0.12] hover:text-white"
									)
								}
							>
								{category}
							</Tab>
						))}
					</Tab.List>
					<Tab.Panels className="mt-2" onFocus={(e: any) => e.target.blur()}>
						<Tab.Panel
							key={"People"}
							className={classNames(
								"rounded-xl bg-white p-3",
								"ring-white ring-opacity-60 ring-offset-2 ring-offset-blue-400 focusId:outline-none focusId:ring-2"
							)}
						>
							{/* {citizens[country.leader]?.name} */}
						</Tab.Panel>
					</Tab.Panels>
				</Tab.Group>
			</div>
		</div>
	);
};
