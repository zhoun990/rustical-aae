import { useEffect, useState } from "react";
import { client } from "../client";

export const SavedData = () => {
	const [saves, setSaves] = useState<Array<string[]>>([]);

	useEffect(() => {
		client.query(["app.getGameData"]).then((vec) => {
			console.log("^_^ Log \n file: App.tsx:29 \n vec:", vec);
			setSaves(
				(vec as Array<string[]>).sort((a, b) => {
					const date1 = new Date(a[1]);
					const date2 = new Date(b[1]);
					return date2.getTime() - date1.getTime();
				})
			);
		});
	}, []);
	return (
		<div>
			{saves.map((id, i) => (
				<div
					key={id[0]}
					className="m-2 p-2 border rounded-lg text-2xl"
					onClick={() => {
						client.query(["app.selectGameId", id[0]]);
					}}
				>
					{id[0]}

					<div className="text-sm">{new Date(id[1]).toLocaleString()}</div>
				</div>
			))}
		</div>
	);
};
