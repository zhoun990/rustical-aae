import { SetStateAction, Suspense, useEffect, useState } from "react";
import { N } from "../../utils/NumericUtils";
import { client, useMutation, useMutationState } from "../../client";
import { BattleState, Cell } from "../../types/rspc/bindings";
type Unit = {
	/** 次の行動までのターン数 */
	defaultDelay: number;
	/** 兵士数。HP、攻撃力、士気に影響 */
	defaultManpower: number;
	/** 攻撃力。敵ユニットの防御力との比でダメージが決まる。 */
	defaultAttack: number;
	/** 防御力。敵ユニットの攻撃力との比で被ダメージが決まる。 */
	defaultDefense: number;
	/** 回避性能。この割合で攻撃を回避しダメージが通らない。0のときは回避せず、1のときは被弾しない。
	 * 0<=n<1 */
	defaultEvasionRate: number;
	canMove?: boolean;
};
// type Cell = {
// 	/** 次の行動までのターン数 */
// 	delay: number;
// 	/** 兵士数。HP、攻撃力、士気に影響 */
// 	manpower: number;
// 	/** 攻撃力。敵ユニットの防御力との比でダメージが決まる。 */
// 	attack: number;
// 	/** 防御力。敵ユニットの攻撃力との比で被ダメージが決まる。 */
// 	defense: number;
// 	/** 回避性能。この割合で攻撃を回避しダメージが通らない。0のときは回避せず、1のときは被弾しない。
// 	 * 0<=n<1 */
// 	evasionRate: number;
// 	type: UnitType;
// 	player: number;
// 	message?: string;
// } | null;
type Board = Cell[][];
type Position = { y: number; x: number };
type SelectedPiece = Position & Cell;

type UnitType = "Infantry" | "Cavalry" | "Artillery" | "Mage";
type Node = {
	children: Node[];
	board: Board;
	round: number;
	isAttacker: boolean;
};

const SIZE = 120;
const ATTACKER = 2;
const DEFENDER = 1;
const row = 9;
const col = 7;
// const units: Record<UnitType, Unit> = {
// 	Infantry: (() => {
// 		return {
// 			defaultDelay: 3,
// 			defaultManpower: 300,
// 			defaultAttack: 30,
// 			defaultDefense: 30,
// 			defaultEvasionRate: 0.2,
// 			canMove: true,
// 		};
// 	})(),
// 	Cavalry: (() => {
// 		return {
// 			defaultDelay: 1,
// 			defaultManpower: 200,
// 			defaultAttack: 40,
// 			defaultDefense: 20,
// 			defaultEvasionRate: 0.3,
// 			canMove: true,
// 		};
// 	})(),
// 	Artillery: (() => {
// 		return {
// 			defaultDelay: 5,
// 			defaultManpower: 100,
// 			defaultAttack: 50,
// 			defaultDefense: 100,
// 			defaultEvasionRate: 0.1,
// 			canMove: true,
// 		};
// 	})(),
// 	Mage: (() => {
// 		return {
// 			defaultDelay: 1,
// 			defaultManpower: 10,
// 			defaultAttack: 40,
// 			defaultDefense: 200,
// 			defaultEvasionRate: 0.5,
// 			canMove: true,
// 		};
// 	})(),
// };
// const u = [
// 	(player: number): Cell => ({
// 		type: "Artillery",
// 		player,
// 		delay: 0,
// 		manpower: units.Artillery.defaultManpower,
// 		attack: units.Artillery.defaultAttack,
// 		defense: units.Artillery.defaultDefense,
// 		evasionRate: units.Artillery.defaultEvasionRate,
// 	}),
// 	(player: number): Cell => ({
// 		type: "Cavalry",
// 		player,
// 		delay: 0,
// 		manpower: units.Cavalry.defaultManpower,
// 		attack: units.Cavalry.defaultAttack,
// 		defense: units.Cavalry.defaultDefense,
// 		evasionRate: units.Cavalry.defaultEvasionRate,
// 	}),
// 	(player: number): Cell => ({
// 		type: "Infantry",
// 		player,
// 		delay: 0,
// 		manpower: units.Infantry.defaultManpower,
// 		attack: units.Infantry.defaultAttack,
// 		defense: units.Infantry.defaultDefense,
// 		evasionRate: units.Infantry.defaultEvasionRate,
// 	}),
// 	(player: number): Cell => ({
// 		type: "Mage",
// 		player,
// 		delay: 0,
// 		manpower: units.Mage.defaultManpower,
// 		attack: units.Mage.defaultAttack,
// 		defense: units.Mage.defaultDefense,
// 		evasionRate: units.Mage.defaultEvasionRate,
// 	}),
// ];
// const n = null;
// const initialBoard: Board = [
// 	[u[0](1), u[1](1), n, n, n, n, n, u[1](2), u[0](2)],
// 	[u[3](1), u[2](1), n, n, n, n, n, u[2](2), u[3](2)],
// 	[u[0](1), u[2](1), n, n, n, n, n, u[2](2), u[0](2)],
// 	[u[3](1), u[1](1), n, n, n, n, n, u[1](2), u[3](2)],
// 	[u[0](1), u[2](1), n, n, n, n, n, u[2](2), u[0](2)],
// 	[u[3](1), u[2](1), n, n, n, n, n, u[2](2), u[3](2)],
// 	[u[0](1), u[1](1), n, n, n, n, n, u[1](2), u[0](2)],
// ];

// const moves: Record<UnitType, Position[]> = {
// 	Infantry: [
// 		{ y: -1, x: 0 },
// 		{ y: 1, x: 0 },
// 		{ y: 0, x: 1 },
// 		{ y: 0, x: -1 },
// 	],
// 	Cavalry: [
// 		{ y: -1, x: -1 },
// 		{ y: 0, x: -2 },
// 		{ y: 1, x: -1 },
// 		{ y: 0, x: 1 },
// 	],
// 	Artillery: [
// 		{ y: -1, x: -1 },
// 		{ y: 0, x: -1 },
// 		{ y: 1, x: -1 },
// 		{ y: 0, x: 1 },
// 	],
// 	Mage: [
// 		{ y: -1, x: -1 },
// 		{ y: -1, x: 0 },
// 		{ y: -1, x: 1 },
// 		{ y: 1, x: -1 },
// 		{ y: 1, x: 1 },
// 		{ y: 1, x: 0 },
// 		{ y: 0, x: 1 },
// 		{ y: 0, x: -1 },
// 		{ y: -2, x: 0 },
// 		{ y: 2, x: 0 },
// 		{ y: 0, x: 2 },
// 		{ y: 0, x: -2 },
// 	],
// };
// const attacks: Record<UnitType, Position[]> = {
// 	Infantry: [
// 		{ y: 0, x: 1 },
// 		{ y: 0, x: -1 },
// 	],
// 	Cavalry: [
// 		{ y: -1, x: -1 },
// 		{ y: 1, x: -1 },
// 	],
// 	Artillery: [
// 		{ y: -1, x: -2 },
// 		{ y: 0, x: -3 },
// 		{ y: 1, x: -2 },
// 	],
// 	Mage: [
// 		{ y: -1, x: -1 },
// 		{ y: -1, x: 0 },
// 		{ y: -1, x: 1 },
// 		{ y: 1, x: -1 },
// 		{ y: 1, x: 1 },
// 		{ y: 1, x: 0 },
// 		{ y: 0, x: 1 },
// 		{ y: 0, x: -1 },
// 	],
// };
// const castAttacks = (pieceType: UnitType, isAttacker: boolean): Position[] => {
// 	const standardMoves = attacks[pieceType] ? attacks[pieceType] : [];
// 	return standardMoves.map((move) => {
// 		return isAttacker
// 			? move
// 			: {
// 					...move,
// 					y: -move.y,
// 					x: -move.x,
// 			  };
// 	});
// };
// const castMoves = (pieceType: UnitType, isAttacker: boolean): Position[] => {
// 	const standardMoves = moves[pieceType] ? moves[pieceType] : [];
// 	return standardMoves.map((move) => {
// 		return isAttacker
// 			? move
// 			: {
// 					...move,
// 					y: -move.y,
// 					x: -move.x,
// 			  };
// 	});
// };
// const getPossibleMoves = (
// 	board: Board,
// 	selected: SelectedPiece
// ): Position[] => {
// 	const moveDefinitions = castMoves(
// 		selected.type,
// 		selected.player === ATTACKER
// 	);
// 	return moveDefinitions.flatMap((move) => {
// 		if (selected.delay !== 0) return [];
// 		const newY = selected.y + move.y;
// 		const newX = selected.x + move.x;
// 		if (
// 			newY < 0 ||
// 			newY >= board.length ||
// 			newX < 0 ||
// 			newX >= board[0].length
// 		) {
// 			return [];
// 		}

// 		const destinationCell = board[newY][newX];
// 		if (destinationCell) {
// 			// 駒がその場所にある
// 			return [];
// 		}
// 		return [{ y: newY, x: newX }];
// 	});
// };
// const getPossibleAttacks = (
// 	board: Board,
// 	selected: SelectedPiece,
// 	ignoreSelfPiece?: boolean
// ): Position[] => {
// 	const attackDefinitions = castAttacks(
// 		selected.type,
// 		selected.player === ATTACKER
// 	);
// 	return attackDefinitions.flatMap((move) => {
// 		const newY = selected.y + move.y;
// 		const newX = selected.x + move.x;
// 		if (
// 			newY < 0 ||
// 			newY >= board.length ||
// 			newX < 0 ||
// 			newX >= board[0].length
// 		) {
// 			return [];
// 		}

// 		const destinationCell = board[newY][newX];
// 		if (
// 			!ignoreSelfPiece &&
// 			destinationCell &&
// 			destinationCell.player === selected.player
// 		) {
// 			// 自分の駒がその場所にある
// 			return [];
// 		}
// 		return [{ y: newY, x: newX }];
// 	});
// };
// const evaluateBoard = (board: Board) => {
// 	const score = { attacker: 0, defender: 0 };
// 	board.forEach((row, y) => {
// 		row.forEach((cell, x) => {
// 			if (cell) {
// 				const player = cell.player === ATTACKER ? "attacker" : "defender";
// 				score[player] += cell.manpower;
// 				score[player] += cell.attack * 10;
// 				score[player] += cell.defense * 10;
// 				score[player] += cell.evasionRate * 100;
// 			}
// 		});
// 	});
// 	return score.attacker - score.defender;
// };
// const attackSimulate = (board: Board, round: number) =>
// 	board.map((_x, y) =>
// 		_x.map((cell, x) => {
// 			if (cell) {
// 				if (
// 					cell &&
// 					(Math.max(0, round - 1) % 2 === 0
// 						? ATTACKER === cell.player
// 						: ATTACKER !== cell.player)
// 				) {
// 					const possibleAttacks: Position[] = [];
// 					const attackDefinitions = castAttacks(
// 						cell.type,
// 						cell.player === ATTACKER
// 					);
// 					getPossibleAttacks(board, { ...cell, x, y }).forEach(
// 						(possibleAttack) => {
// 							const enemy = board[possibleAttack.y]?.[possibleAttack.x];
// 							if (enemy && enemy.player !== cell.player) {
// 								possibleAttacks.push(possibleAttack);
// 							}
// 						}
// 					);
// 					possibleAttacks.forEach((target) => {
// 						const enemy = board[target?.y]?.[target?.x];
// 						if (target && enemy) {
// 							board[target.y][target.x]!.manpower -=
// 								(cell.attack *
// 									(cell.attack / enemy.defense) *
// 									(1 - enemy.evasionRate)) /
// 								possibleAttacks.length;

// 							if (board[target.y][target.x]!.manpower < 0) {
// 								board[target.y][target.x] = null;
// 							}
// 						}
// 					});
// 				}
// 			}
// 			return cell;
// 		})
// 	);
// const attack = (board: Board, round: number) =>
// 	board.map((_x, y) =>
// 		_x.map((cell, x) => {
// 			if (cell) {
// 				cell.message = "";
// 				if (
// 					Math.max(0, round - 1) % 2 === 0
// 						? ATTACKER === cell.player
// 						: ATTACKER !== cell.player
// 				) {
// 					const possibleAttacks: Position[] = [];
// 					getPossibleAttacks(board, { ...cell, x, y }).forEach(
// 						(possibleAttack) => {
// 							const enemy = board[possibleAttack.y]?.[possibleAttack.x];
// 							if (enemy && enemy.player !== cell.player) {
// 								possibleAttacks.push(possibleAttack);
// 							}
// 						}
// 					);
// 					const target = N.pick(possibleAttacks);
// 					const enemy = board[target?.y]?.[target?.x];
// 					if (target && enemy) {
// 						if (N.percentage(enemy.evasionRate, 1)) {
// 							board[target.y][target.x]!.manpower -=
// 								cell.attack * (cell.attack / enemy.defense);

// 							board[target.y][target.x]!.message = "hit!!";
// 							cell.message = "attacked!!";
// 						} else {
// 							board[target.y][target.x]!.message = "evasion!!";
// 						}
// 						if (board[target.y][target.x]!.manpower < 0) {
// 							board[target.y][target.x] = null;
// 						}
// 					}
// 				}
// 			}
// 			return cell;
// 		})
// 	);
// const createNewBoard = ({
// 	board,
// 	from,
// 	to,
// }: {
// 	board: Board;
// 	from: Position;
// 	to: Position;
// }): Board => {
// 	const selectedCell = board[from.y][from.x];
// 	if (!selectedCell) return board;
// 	return board.map((row, y) =>
// 		row.map((cell, x) => {
// 			if (y === to.y && x === to.x) {
// 				return {
// 					...selectedCell,
// 					delay: Math.max(0, units[selectedCell.type].defaultDelay - 1),
// 				};
// 			}
// 			if (!cell || (y === from.y && x === from.x)) {
// 				return null;
// 			}

// 			return {
// 				...cell,
// 				delay: Math.max(0, cell.delay - 1),
// 			};
// 		})
// 	);
// };
// const move = ({
// 	board,
// 	from,
// 	to,
// 	round,
// }: {
// 	board: Board;
// 	from: Position;
// 	to: Position;
// 	round: number;
// }) => {
// 	const selectedCell = board[from.y][from.x];
// 	if (!selectedCell) return board;
// 	let newBoard: Board = createNewBoard({ board, from, to });
// 	return attackSimulate(newBoard, round);
// };
// const getSelectableCells = (board: Board, round: number) => {
// 	const player = round % 2 === 0 ? ATTACKER : DEFENDER;
// 	let cells: (Position & { possibleMoves: Position[] })[] = [];
// 	board.forEach((_x, y) =>
// 		_x.forEach((_y, x) => {
// 			const cell = board[y]?.[x];
// 			const possibleMoves = cell && getPossibleMoves(board, { ...cell, y, x });
// 			const isSelectable =
// 				board[y]?.[x]?.player === player && possibleMoves?.length;
// 			if (cell && isSelectable) {
// 				cells.push({ y, x, possibleMoves });
// 			}
// 		})
// 	);
// 	return cells;
// };
// function minimax(node: Node, depth: number) {
// 	return alphabeta(node, depth, -Infinity, Infinity);
// }
// const alphabeta = (node: Node, depth: number, a: number, b: number) => {
// 	if (depth === 0) {
// 		return evaluateBoard(node.board);
// 	}
// 	const cells = getSelectableCells(node.board, node.round);
// 	const children = cells.flatMap((from) =>
// 		from.possibleMoves.map((to) => ({
// 			children: [],
// 			board: move({
// 				board: node.board,
// 				from: { x: from.x, y: from.y },
// 				to: { x: to.x, y: to.y },
// 				round: node.round,
// 			}),
// 			round: node.round + 1,
// 			isAttacker: node.isAttacker,
// 		}))
// 	);
// 	if (children.length === 0) {
// 		return evaluateBoard(node.board);
// 	}
// 	const isOwnNode = node.round % 2 === 0 ? node.isAttacker : !node.isAttacker;
// 	if (isOwnNode) {
// 		for (const child of children) {
// 			a = Math.max(a, alphabeta(child, depth - 1, a, b));
// 			if (a >= b) {
// 				break; // βカット
// 			}
// 		}
// 		return a;
// 	} else {
// 		for (const child of children) {
// 			b = Math.min(b, alphabeta(child, depth - 1, a, b));
// 			if (a >= b) {
// 				break; // αカット
// 			}
// 		}
// 		return b;
// 	}
// };
// function sleep(ms: number): Promise<void> {
// 	return new Promise((resolve) => {
// 		setTimeout(resolve, ms);
// 	});
// }
export const BattleScreen = () => {
	const m = window.innerWidth - SIZE * row;
	const [board, setBoard] = useState<(Cell | null)[][]>([]);
	const [round, setRound] = useState(0);
	const [battleData, setBattleData] = useState({
		attacker_id: -1,
		defender_id: -1,
		round: 0,
	});
	const [aiPlayer, setAiPlayer] = useState(true);
	const [isProcessing, setIsProcessing] = useState(false);
	const [autoPlayEnabled, setAutoPlayEnabled] = useState(false);
	const [activeAttacks, setActiveAttacks] = useState<Position[]>([]);
	const [possibleBeAttacked, setPossibleBeAttacked] = useState<Position[]>([]);
	const [autoPlayContinueEnabled, setAutoPlayContinueEnabled] = useState(false);
	const [selectedPiece, setSelectedPiece] = useState<SelectedPiece | null>(
		null
	);

	// const aiHandler = async () => {
	// 	setIsProcessing(true);
	// 	if (!autoPlayContinueEnabled) await sleep(1000);
	// 	const cells = getSelectableCells(board, round);
	// 	const nodes = cells.flatMap((from) =>
	// 		from.possibleMoves.map((to) => ({
	// 			children: [],
	// 			board: move({
	// 				board,
	// 				from: { x: from.x, y: from.y },
	// 				to: { x: to.x, y: to.y },
	// 				round: round,
	// 			}),
	// 			from: { x: from.x, y: from.y },
	// 			to: { x: to.x, y: to.y },
	// 			round: round + 1,
	// 			isAttacker: round % 2 === 0,
	// 			score: 0,
	// 		}))
	// 	);
	// 	nodes.forEach((node) => {
	// 		node.score = minimax(node, 3);
	// 	});
	// 	let highest = [nodes[0]];
	// 	for (let i = 1; i < nodes.length; i++) {
	// 		if (
	// 			round % 2 === 0
	// 				? nodes[i].score > highest[0].score
	// 				: nodes[i].score < highest[0].score
	// 		) {
	// 			highest = [nodes[i]];
	// 		} else if (nodes[i].score === highest[0].score) {
	// 			highest.push(nodes[i]);
	// 		}
	// 	}
	// 	const node = N.pick(highest);
	// 	console.log("^_^ Log \n file: BattleScreen.tsx:732 \n node:", node);
	// 	if (node) {
	// 		const newBoard: Board = createNewBoard({
	// 			board,
	// 			from: node.from,
	// 			to: node.to,
	// 		});
	// 		setBoard(newBoard);
	// 		setSelectedPiece(null);
	// 		setRound((c) => c + 1);
	// 	} else {
	// 		console.error("AI move error:", node);
	// 		setAutoPlayContinueEnabled(false);
	// 	}
	// 	setIsProcessing(false);
	// 	setAutoPlayEnabled(false);
	// };
	useEffect(() => {
		client.query(["app.battleGet"]).then((res) => {
			console.log("^_^ Log \n file: BattleScreen.tsx:562 \n res:", res);
			setBoard(res.board as (Cell | null)[][]);
			setRound(res.round);
			setBattleData({
				attacker_id: res.attacker_id,
				defender_id: res.defender_id,
				round: 0,
			});
		});
		// client
		// 	.mutation([
		// 		"app.battleMove",
		// 		[
		// 			{ x: 0, y: 0 },
		// 			{ x: 0, y: 0 },
		// 		],
		// 	])
		// 	.then((res) => {});
	}, []);
	useEffect(() => {
		if (autoPlayEnabled) {
			if (isProcessing) {
				setAutoPlayEnabled(false);
			} else {
				// aiHandler();
			}
		}
	}, [autoPlayEnabled]);
	useEffect(() => {
		if (autoPlayContinueEnabled) {
			setAiPlayer(false);
		}
		if (!autoPlayEnabled && autoPlayContinueEnabled) {
			setTimeout(() => {
				setAutoPlayEnabled(true);
			}, 10);
		}
	}, [autoPlayEnabled, autoPlayContinueEnabled]);
	// useEffect(() => {
	// 	console.log("Score:", evaluateBoard(board));
	// }, [board]);
	// const restart = () => {
	// 	setAutoPlayContinueEnabled(false);
	// 	setBoard(initialBoard);
	// 	setRound(0);
	// 	setAutoPlayEnabled(false);
	// 	setSelectedPiece(null);
	// };
	// useEffect(() => {
	// 	if (
	// 		board.reduce(
	// 			(acc, v) => acc + v.filter((v2) => v2 && v2.player === ATTACKER).length,
	// 			0
	// 		) === 0 ||
	// 		board.reduce(
	// 			(acc, v) => acc + v.filter((v2) => v2 && v2.player === DEFENDER).length,
	// 			0
	// 		) === 0 ||
	// 		round > 1000
	// 	) {
	// 		console.log(
	// 			"Game Over, in round:",
	// 			round,
	// 			evaluateBoard(board) >= 0 ? "Attacker Won" : "Defender won"
	// 		);
	// 		restart();
	// 	} else {
	// 		setBoard((newBoard) => attack(newBoard, round));
	// 		if (aiPlayer && round % 2 !== 0) {
	// 			console.log("ai turn");
	// 			setAutoPlayEnabled(true);
	// 		}
	// 	}
	// }, [round]);
	// const [possibleMoves, getPossibleMoves] = useMutationState(
	// 	"app.battleGetPossibleMoves"
	// );
	const [possibleAttacks, getPossibleAttacks] = useMutationState(
		"app.battleGetPossibleAttacks"
	);
	// selectedPiece && getPossibleMoves({ x: selectedPiece.x, y: selectedPiece.y });
	// selectedPiece &&
	// 	getPossibleAttacks({ x: selectedPiece.x, y: selectedPiece.y });
	useEffect(() => {
		if (selectedPiece)
			getPossibleAttacks({ x: selectedPiece.x, y: selectedPiece.y });
	}, [selectedPiece]);
	useEffect(() => {
		setPossibleBeAttacked([]);
		setActiveAttacks([]);
		board.forEach((row, y) =>
			row.forEach(async (cell, x) => {
				if (cell) {
					// client.mutation(["app.battleGetPossibleAttacks", { x, y }]);
					const possibleAttacks = await client.mutation([
						"app.battleGetPossibleAttacks",
						{ x, y },
					]);
					if (battleData.attacker_id !== cell?.owner_id) {
						possibleAttacks.forEach((pos) => {
							if (board[pos.y]?.[pos.x]?.owner_id === battleData.attacker_id) {
								setPossibleBeAttacked((c) => [...c, pos]);
							}
						});
					} else {
						possibleAttacks.forEach((pos) => {
							if (board[pos.y]?.[pos.x]?.owner_id === battleData.defender_id) {
								setActiveAttacks((c) => [...c, pos]);
							}
						});
					}
				}
			})
		);
	}, [board]);
	const turnEnd = () => {
		setBoard((c) =>
			c.map((row, j) =>
				row.map((cell, i) => {
					if (!cell) {
						return null;
					}
					return {
						...cell,
						delay: Math.max(0, cell.delay - 1),
					};
				})
			)
		);
		setSelectedPiece(null);
		setRound((c) => c + 1);
	};
	return (
		<div className="h-screen bg-slate-600 overflow-y-hidden">
			<div className="flex">
				<button className="border m-2 p-1" onClick={turnEnd}>
					turn end
				</button>
				<button
					className="border m-2 p-1"
					style={{ backgroundColor: aiPlayer ? "#ff9f20" : undefined }}
					onClick={() => {
						setAiPlayer((c) => !c);
					}}
				>
					{aiPlayer ? "AI Disable" : "AI Enable"}
				</button>
				<button
					className="border m-2 p-1"
					style={{
						backgroundColor: autoPlayContinueEnabled ? "#ff9f20" : undefined,
					}}
					onClick={() => {
						setAutoPlayContinueEnabled((c) => !c);
					}}
				>
					{autoPlayContinueEnabled ? "Stop" : "Auto play"}
				</button>
			</div>
			<div
				className="flex pl-[0px] items-center bg-slate-600 overflow-y-hidden"
				style={{ paddingLeft: m > 0 ? m / 2 : 0 }}
			>
				<div className="flex">
					{Array(row)
						.fill(0)
						.map((_, x) => (
							<div key={x}>
								{Array(col)
									.fill(0)
									.map((_, y) => {
										// const cell = board[y]?.[x];

										// const isAttacker =
										// 	round % 2 === 0
										// 		? ATTACKER === board[y]?.[x]?.player
										// 		: board[y]?.[x] && ATTACKER !== board[y]?.[x]?.player;
										// const isSelectable =
										// 	possibleMoves?.some(
										// 		(move) => move.y === y && move.x === x
										// 	) ||
										// 	(isAttacker &&
										// 		!selectedPiece &&
										// 		cell &&
										// 		getPossibleMoves(board, { ...cell, y, x }).length);

										return (
											<CellView
												x={x}
												y={y}
												selectedPiece={selectedPiece}
												setSelectedPiece={setSelectedPiece}
												cell={board[y]?.[x]}
												battleData={battleData}
												setRound={setRound}
												setBoard={setBoard}
												possibleBeAttacked={possibleBeAttacked}
												activeAttacks={activeAttacks}
												possibleAttacks={possibleAttacks || []}
												possibleMoves={[]}
											/>
										);
									})}
							</div>
						))}
				</div>
			</div>
		</div>
	);
};
const CellView = ({
	x,
	y,
	selectedPiece,
	setSelectedPiece,
	cell,
	battleData,
	setRound,
	setBoard,
	possibleBeAttacked,
	activeAttacks,
	possibleAttacks,
	possibleMoves,
}: {
	x: number;
	y: number;
	selectedPiece: SelectedPiece | null;
	setSelectedPiece: React.Dispatch<React.SetStateAction<SelectedPiece | null>>;
	cell: Cell | null;
	battleData: { attacker_id: number; defender_id: number; round: number };
	setRound: React.Dispatch<React.SetStateAction<number>>;
	setBoard: React.Dispatch<React.SetStateAction<(Cell | null)[][]>>;
	possibleMoves: Position[];
	possibleAttacks: Position[];
	possibleBeAttacked: Position[];
	activeAttacks: Position[];
}) => {
	// const [possibleMoves, getPossibleMoves] = useMutationState(
	// 	"app.battleGetPossibleMoves"
	// );
	// selectedPiece && getPossibleMoves({ x: selectedPiece.x, y: selectedPiece.y });

	// const [possibleAttacks, getPossibleAttacks] = useMutationState(
	// 	"app.battleGetPossibleMoves"
	// );
	// selectedPiece && getPossibleMoves({ x: selectedPiece.x, y: selectedPiece.y });
	const autoPlayContinueEnabled = false;
	const isAttacker =
		battleData.round % 2 === 0
			? battleData.attacker_id === cell?.owner_id
			: cell && battleData.defender_id === cell?.owner_id;
	const isSelectable =
		possibleMoves?.some((move) => move.y === y && move.x === x) ||
		(isAttacker && !selectedPiece && cell && possibleMoves?.length);
	// const getPossibleAttacks = async ({ x, y }: { x: number; y: number }) =>
	// 	await client.mutation(["app.battleGetPossibleMoves", { x, y }]);
	// const possibleMoves=await getPossibleAttacks({ x, y });
	return (
		<div
			key={y}
			className="flex justify-center items-center flex-col"
			style={(() => {
				const rounded = selectedPiece?.x === x && selectedPiece.y === y;
				const offset = rounded ? 10 : 0;
				const size = SIZE - offset * 2;
				return {
					width: size,
					height: size,
					margin: offset + 1,
					borderRadius: rounded ? 1000 : undefined,
					// borderWidth: BW,
					backgroundColor: possibleAttacks?.some((v) => v.x === x && v.y === y)
						? "#ff9f20"
						: cell
						? battleData.attacker_id === cell?.owner_id
							? "#ADD8E6"
							: "#FFC0CB"
						: (x + y) % 2 === 0
						? "#228B22"
						: "#90EE90",
					color: possibleAttacks?.some((v) => v.x === x && v.y === y)
						? battleData.attacker_id === cell?.owner_id
							? "blue"
							: "red"
						: cell
						? "#464646"
						: (x + y) % 2 !== 0
						? "#228B22"
						: "#90EE90",
					borderWidth:
						selectedPiece && possibleAttacks.some((v) => v.x === x && v.y === y)
							? 3
							: !selectedPiece &&
							  (possibleBeAttacked.some((v) => v.x === x && v.y === y) ||
									activeAttacks.some((v) => v.x === x && v.y === y))
							? 10
							: autoPlayContinueEnabled
							? 0
							: isSelectable
							? 3
							: 0,
					borderColor: isSelectable
						? "white"
						: selectedPiece &&
						  possibleAttacks.some((v) => v.x === x && v.y === y)
						? "#ff9f20"
						: possibleBeAttacked.some((v) => v.x === x && v.y === y)
						? "#FFA07A"
						: activeAttacks.some((v) => v.x === x && v.y === y)
						? "#87CEFA"
						: "white",
				};
			})()}
			onClick={async () => {
				if (selectedPiece && selectedPiece.x === x && selectedPiece.y === y) {
					console.log("deselect piece");
					setSelectedPiece(null);
				} else if (
					cell &&
					(isSelectable ||
						(isAttacker &&
							(await client.mutation(["app.battleGetPossibleMoves", { x, y }]))
								.length &&
							cell?.owner_id === cell?.owner_id))
				) {
					console.log("select piece");
					if (cell.unit_type && cell.owner_id) {
						setSelectedPiece({ ...cell, y, x });
					}
				} else {
					console.log({
						cell,
						isSelectable,
						isAttacker,
						length: (
							await client.mutation(["app.battleGetPossibleMoves", { x, y }])
						).length,
						owner_id: cell?.owner_id,
						owner_id2: cell?.owner_id,
					});
				}
				// else if (selectedPiece) {
				// 	console.log("move piece triggered");
				// 	if (possibleMoves?.some((move) => move.y === y && move.x === x)) {
				// 		console.log("move piece detected");
				// 		client
				// 			.mutation([
				// 				"app.battleMove",
				// 				[
				// 					{
				// 						x: selectedPiece.x,
				// 						y: selectedPiece.y,
				// 					},
				// 					{ x, y },
				// 				],
				// 			])
				// 			.then((res) => {
				// 				console.log(
				// 					"^_^ Log \n file: BattleScreen.tsx:562 \n res:",
				// 					res
				// 				);
				// 				setBoard(res.board as (Cell | null)[][]);
				// 				setRound(res.round);
				// 				setSelectedPiece(null);
				// 			});
				// 	}
				// } else {
				// 	console.log("no piece selected");
				// }
			}}
		>
			{/* x:{x},y:{y}, */}

			<div>{cell?.unit_type}</div>
			<div>{Math.round((cell?.delay || 0) / 2) || ""}</div>
			<div>{Math.round(cell?.manpower || 0) || ""}</div>
			<div>{cell?.message || ""}</div>
		</div>
	);
};
