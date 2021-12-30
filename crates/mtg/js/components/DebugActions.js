import { useContext } from "react";

import { GameContext } from "./GameRoot";

export default function DebugActions({ player }) {
	const { doAction } = useContext(GameContext);

	const concede = () => doAction(player, { type: "Concede" });
	const passPriority = () => doAction(player, { type: "PassPriority" });
	const noAttackers = () => doAction(player, { type: "ChooseAttackers", attackers: [] });
	const noBlockers = () => doAction(player, { type: "ChooseBlockers", blockers: [] });

	return (
		<div>
			<button onClick={concede}>Concede</button>
			<button onClick={passPriority}>Pass Priority</button>
			<button onClick={noAttackers}>No Attackers</button>
			<button onClick={noBlockers}>No Blockers</button>
		</div>
	);
}