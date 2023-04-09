import { Tab } from "@headlessui/react";
import { FC, useContext, useEffect, useState } from "react";
import { Store } from "../../store";
import { Tooltip } from "../Interface/Tooltip";
import { RegionClass } from "../../core/Region";
import { classNames } from "../../utils/classNames";
import { N } from "../../utils/NumericUtils";
import { CITY_NAME_LIST } from "../../utils/constans";
import { CityClass } from "../../core/City";
// import { Configuration, OpenAIApi } from "openai";

export const StatePanel: FC<{ editable?: boolean }> = ({ editable }) => {
	const { focus, regions, set } = useContext(Store);
	const [type, setType] = useState<"state" | "country">("state");
	const [name, setName] = useState("");
	const add = () => {
		// if (name && editable) {
		// 	const id = generateUniqueId(20);
		// 	Settlement.addSettlement({
		// 		name,
		// 		position: state.self.position,
		// 		id,
		// 		state: state.self.id,
		// 	});
		// 	setName("");
		// }
	};
	const focusId = focus.id;
	const region = (() => {
		if (focusId) {
			const r = regions.get(focusId);
			if (r) return new RegionClass(r.id);
		}
		return undefined;
	})();
	if (!region || !focusId) {
		return null;
	}
	console.log("^_^ Log \n file: StatePanel.tsx:38 \n region:", region);

	return (
		<div className="fixed w-[700px] top-0 left-3 h-[100%] py-3">
			<div className="bg-gray-200 p-3 h-[100%] rounded-lg overflow-y-scroll">
				<div className="flex">
					<Tooltip className="text-[30px]" content="State info">
						State
					</Tooltip>
					{/* {!isProd && `[${state.self.id}]`} */}

					<input
						type="text"
						value={region.self.name}
						className="grow ml-3 pl-3 pr-3"
						onChange={(e) => {
							region.update((c) => {
								c.name = e.target.value;
								return c;
							});
							// set({
							// 	regions: (value) => {
							// 		region.self.name = e.target.value;
							// 		return new Map([...value, [focusId, region.self]]);
							// 	},
							// });
						}}
					/>
					{/* <button
                onClick={() => {
                    //dispatch(actions.increaseTime());
                }}
            >
                button{time}
            </button> */}
				</div>
				<div className="flex h-[70px]">
					{(["building", "production", "infrastructure"] as const).map(
						(type) => {
							// if (settlement.devExp[type] >= ) {
							// 	settlement.devExp[type] -= state.dev[type] ** 2 * 1000;
							// 	state.dev[type]++;
							// 	}
							return (
								<div key={type} className="center flex-col w-[115px]">
									<div className="text-[13px]">{type}</div>
									<div className="text-[20px] leading-3">
										{region.dev[type]}
									</div>
								</div>
							);
						}
					)}
					<div className="flex">
						<div className="grow center flex-col mx-1 w-[100px]">
							<div className="text-[13px]">produce</div>
							<div className="text-[20px] leading-3">{region.self.product}</div>
						</div>
						<Tooltip
							className="grow center flex-col mx-1"
							content="Environment of State"
						>
							<div className="text-[13px]">env</div>
							<div className="text-[20px] leading-3">{region.environment}</div>
						</Tooltip>
					</div>
				</div>
				<Tab.Group
					manual
					defaultIndex={localStorage.StatePanelSettlementTabIndex}
					onChange={(index) =>
						(localStorage.StatePanelSettlementTabIndex = index)
					}
				>
					<Tab.List
						className="flex space-x-1 rounded-xl bg-blue-900/20 p-1 overflow-x-auto"
						onFocus={(e: any) => e.target.blur()}
					>
						{region.cities.map((city, i) => (
							<Tab
								key={city.id}
								className={({ selected }) =>
									classNames(
										"w-full rounded-lg py-2.5 text-sm font-medium leading-5 text-blue-700",
										"ring-white ring-opacity-60 ring-offset-2 ring-offset-blue-400 focus:outline-none focus:ring-2",
										selected
											? "bg-white shadow"
											: "text-blue-100 hover:bg-white/[0.12] hover:text-white"
									)
								}
							>
								{city.self.name}
							</Tab>
						))}

						{editable && (
							<Tab
								key={"add"}
								className={({ selected }) =>
									classNames(
										"w-full rounded-lg py-2.5 text-sm font-medium leading-5 text-blue-700",
										"ring-white ring-opacity-60 ring-offset-2 ring-offset-blue-400 focus:outline-none focus:ring-2",
										selected
											? "bg-white shadow"
											: "text-blue-100 hover:bg-white/[0.12] hover:text-white"
									)
								}
							>
								Add
							</Tab>
						)}
					</Tab.List>
					<Tab.Panels className="mt-2">
						{Object.entries(region.cities).map(([key, city], i) => {
							// const sumControl = state.self.settlements
							// 	.map((key) => settlements[key].control)
							// 	.reduce((sum, value) => sum + value, 0);
							// const controlRate = city.control / sumControl;
							// const building = state.self.dev.building * controlRate;
							// const stockpileCapacity = Math.floor(
							// 	building *
							// 		((100 - city.residentialLandRate) / 100) *
							// 		((1000 + building) / 10)
							// );
							const joinArmy = (id: number) => {
								// dispatch(
								// 	actions.set({
								// 		armies: (value) => {
								// 			value[id] = {
								// 				id,
								// 				member: [id],
								// 				state: "awaiting",
								// 				action: [],
								// 				location: focus.id,
								// 			};
								// 			return value;
								// 		},
								// 	})
								// );
							};
							return (
								<Tab.Panel key={i} className={"rounded-xl bg-white p-3"}>
									<SettlementInfo city={city} editable={editable} />
								</Tab.Panel>
							);
						})}
						<Tab.Panel key={"add"} className={"rounded-xl bg-white p-3"}>
							<div>Name</div>
							<div className="flex mb-3">
								<input
									type="text"
									className="border w-full"
									value={name}
									onChange={(e) => setName(e.target.value)}
								/>
								<button
									className="p-1 border rounded outline-none"
									onFocus={(e) => e.target.blur()}
									onClick={() => {
										setName(N.pick(CITY_NAME_LIST));
									}}
								>
									Random
								</button>
							</div>
							<button
								className="p-1 border rounded outline-none"
								onFocus={(e) => e.target.blur()}
								onClick={add}
							>
								Add A New Settlement
							</button>
						</Tab.Panel>
					</Tab.Panels>
				</Tab.Group>
			</div>
		</div>
	);
};
const SettlementInfo = ({
	city,
	editable,
}: {
	city: CityClass;
	editable?: boolean;
}) => {
	const { set } = useContext(Store);

	// const { itemTable } = useContext(Store);
	const joinArmy = (id: number) => {
		// dispatch(
		// 	actions.set({
		// 		armies: (value) => {
		// 			value[id] = {
		// 				id,
		// 				member: [id],
		// 				state: "awaiting",
		// 				action: [],
		// 				location: focus.id,
		// 			};
		// 			return value;
		// 		},
		// 	})
		// );
	};
	const talk = (person: any) => {
		console.log("^_^ Log \n file: StatePanel.tsx:95 \n person", person);

		// window.mainAPI
		// 	.openAi(
		// 		`This is your profile.

		// ${JSON.stringify(person)}

		// First, tell your name. And greet.
		// Then Provide three Yes-No questions about your life in the world to pick one and ask later.`
		// 	)
		// 	.then((res) => {
		// 		res.choices.forEach((value) => {
		// 			console.log(value.text);
		// 		});
		// 	});
	};
	return (
		<div className={"rounded-xl bg-white p-3"}>
			<Tooltip
				className="grow  flex-col mx-1 text-2xl"
				content="Environment of State"
			>
				<input
					type="text"
					value={city.self.name}
					onChange={(e) => {
						city.update((c) => {
							c.name = e.target.value;
							return c;
						});
					}}
				/>
			</Tooltip>
			<div className="flex h-[70px]">
				{(["building", "production", "infrastructure"] as const).map((type) => {
					const editDev = (isPos: boolean) => {
						city.update((c) => {
							c[`dev_${type}`] = Math.max(
								1,
								c[`dev_${type}`] + (isPos ? 1 : -1)
							);
							return c;
						});
					};
					return (
						<div key={type} className="center flex-col w-[115px]">
							<div className="text-[13px]">{type}</div>
							<div className="text-[20px] leading-3">
								<Tooltip
									content={
										<div>
											<div className="text-xl">development</div>
											<div>{city.self[`dev_${type}`]}</div>
											<div className="text-xl">
												to the next degree of development
											</div>
											<div>
												{N.round(
													city.self[`dev_${type}`] ** 2 * 1000 -
														city.self[`exp_dev_${type}`],
													3
												)}
											</div>
										</div>
									}
								>
									{city.self[`dev_${type}`]}[
									{N.formatNumber(city.self[`exp_dev_${type}`])}/
									{N.formatNumber(city.self[`dev_${type}`] ** 2 * 1000)}]
								</Tooltip>

								{editable && (
									<button
										className="border rounded px-1 ml-1 bg-[#fd9999]"
										onClick={() => editDev(false)}
									>
										-
									</button>
								)}
								{editable && (
									<button
										className="border rounded px-1 ml-1 bg-[#99fd99]"
										onClick={() => editDev(true)}
									>
										+
									</button>
								)}
							</div>
						</div>
					);
				})}
				<div className="flex">
					<div className="grow center flex-col mx-1 w-[100px]">
						<div className="text-[13px]">produce</div>
						<div className="text-[20px] leading-3">
							{city.region.self.product}
						</div>
					</div>
				</div>
				<div className="flex">
					<div className="grow center flex-col mx-1 w-[100px]">
						<div className="text-[13px]">State Control </div>
						<div className="text-[20px] leading-3">
							{editable ? (
								<input
									type="number"
									value={N.round(city.self.control, 2)}
									onChange={(e) => {
										city.update((c) => {
											c.control = Number(e.target.value);
											return c;
										});
									}}
								/>
							) : (
								<div>{N.round(city.self.control)}</div>
							)}
						</div>
					</div>
				</div>
			</div>
			<div className="flex h-[70px]">
				<div className="flex">
					<div className="grow center flex-col mx-1">
						<div className="text-[13px]">Country</div>
						<div className="text-[20px] leading-3">
							<button
								className="border-2 rounded p-1 font-bold"
								onClick={() => {
									if (city.self.country_id) {
										set({
											focus: {
												type: "country",
												id: city.self.country_id,
											},
										});
									}
								}}
							>
								{/* {city.country?.self.name ||
									(editable && (
										<button
											className="button"
											onClick={() => {
												set({
													selectedSettlements:(value) =>
													value.concat(city.id),
												});
											}}
										>
											Establish a new country
										</button>
									)) ||
									"not owned"} */}
							</button>
						</div>
					</div>
				</div>
				{/* <div className="flex">
					<div className="grow center flex-col mx-1 w-[100px]">
						<div className="text-[13px]">TP</div>
						<div className="text-[20px] leading-3">
							{city.self.building["trading-post"].level || "None"}
						</div>
					</div>
				</div> */}
				<div className="flex">
					<div className="grow center flex-col mx-1 w-[100px]">
						<div className="text-[13px]">Environment</div>
						<div className="text-[20px] leading-3">
							{N.round(city.self.environment)}
						</div>
					</div>
				</div>
			</div>

			{/* <div>
				<div>
					people needing food:
					<br />
					<input
						type="range"
						min="0"
						max={10}
						value={settlement.self.stockpileNumberOfPeople}
						onChange={(e) => {
							settlement.set((self) => {
								self.stockpileNumberOfPeople = Number(e.target.value);
								self.stockpileMonths = Math.min(
									self.stockpileMonths,
									Math.floor(
										settlement.stockpileCapacity /
											(settlement.self.stockpileNumberOfPeople || 1)
									)
								);
							});
						}}
					/>
					{settlement.self.stockpileNumberOfPeople}
				</div>
				Stockpile amount(months):
				<br />
				<input
					type="range"
					min="0"
					max={Math.floor(
						settlement.stockpileCapacity /
							(settlement.self.stockpileNumberOfPeople || 1)
					)}
					value={settlement.self.stockpileMonths}
					onChange={(e) => {
						settlement.set((self) => {
							self.stockpileMonths = Number(e.target.value);
						});
					}}
				/>
				{settlement.self.stockpileMonths}
			</div> */}
			{/* <Popover>
<Popover.Button ref={setReferenceElement}>Solutions</Popover.Button>

<Popover.Panel
ref={setPopperElement}
style={{ ...styles.popper }}
className="bg-white"
{...attributes.popper}
>
Popover Popover Popover Popover Popover Popover Popover Popover
Popover Popover Popover Popover Popover Popover
</Popover.Panel>
</Popover> */}

			<Tab.Group
				manual
				defaultIndex={localStorage.StatePanelTabIndex}
				onChange={(index) => (localStorage.StatePanelTabIndex = index)}
			>
				<Tab.List
					className="flex space-x-1 rounded-xl bg-blue-900/20 p-1"
					onFocus={(e: any) => e.target.blur()}
				>
					{["People", "Production"].map((category) => (
						<Tab
							key={category}
							className={({ selected }) =>
								classNames(
									"w-full rounded-lg py-2.5 text-sm font-medium leading-5 text-blue-700",
									"ring-white ring-opacity-60 ring-offset-2 ring-offset-blue-400 focus:outline-none focus:ring-2",
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
				<Tab.Panels
					className="mt-2 select-none"
					// onFocus={(e: any) => e.target.blur()}
				>
					<Tab.Panel
						key={"People"}
						className={classNames(
							"rounded-xl bg-white p-3",
							"ring-white ring-opacity-60 ring-offset-2 ring-offset-blue-400 focus:outline-none focus:ring-2"
						)}
					>
						{city.citizens.map((citizen) => {
							return (
								<div key={citizen.id}>
									{/* {!isProd && `[${person}]`} */}
									<input
										type="text"
										value={citizen.self.name}
										onClick={(e) => {
											e.stopPropagation();
										}}
										onChange={(e) => {
											citizen.update((c) => {
												c.name = e.target.value;
												return c;
											});
										}}
									/>
									|age:
									{citizen.age}
									<button
										className="border px-1"
										onClick={() => {
											// joinArmy(id);
										}}
									>
										Join Army
									</button>
									<button
										className="border px-1"
										onClick={() => {
											talk(citizen.self);
										}}
									>
										Talk
									</button>
								</div>
							);
						})}
					</Tab.Panel>
					<Tab.Panel
						key={"Production"}
						className={classNames(
							"rounded-xl bg-white p-3",
							"ring-white ring-opacity-60 ring-offset-2 ring-offset-blue-400 focus:outline-none focus:ring-2"
						)}
					>
						<div className="flex flex-wrap">
							{/* {getObjectKeys(city.self.inventory).map((item) => (
								<Item key={item} settlement={city} item={item} />
							))} */}
						</div>
					</Tab.Panel>
				</Tab.Panels>
			</Tab.Group>
		</div>
	);
};
// const Item = ({
// 	settlement,
// 	item,
// }: {
// 	settlement: Settlement;
// 	item: Items;
// }) => {
// 	const { itemTable } = useContext(Store);
// 	const imgs: Set<Items> = new Set(["fruit", "vegetable", "animal", "grain"]);

// 	return (
// 		<Tooltip
// 			className="w-[115px] center flex-col mb-2"
// 			// width={500}
// 			content={
// 				<div>
// 					<img
// 						src={
// 							imgs.has(item)
// 								? `assets/item_${item}.png`
// 								: "assets/item_fruit.png"
// 						}
// 						alt=""
// 						className="w-[400px] rounded-lg"
// 					/>
// 					<div className="text-3xl whitespace-nowrap w-full overflow-hidden text-ellipsis">
// 						{settlement.self.inventory[item]?.name}
// 					</div>
// 				</div>
// 			}
// 		>
// 			<div className="text-xl whitespace-nowrap w-full overflow-hidden text-ellipsis text-center">
// 				{settlement.self.inventory[item]?.name}
// 			</div>
// 			<div>
// 				<img
// 					src={
// 						imgs.has(item) ? `assets/item_${item}.png` : "assets/item_fruit.png"
// 					}
// 					alt=""
// 					className="w-[100px] rounded-lg"
// 				/>
// 			</div>
// 			{N.round(settlement.self.inventory[item]?.count)}[
// 			{N.round(settlement.self.inventory[item]?.value)}/
// 			{N.round(itemTable[item].value)}]
// 		</Tooltip>
// 	);
// };
