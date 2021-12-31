import { createContext, useState } from "react";

export const GameContext = createContext(null);

export default function GameRoot({ game, children }) {
	const [num, rerender] = useState(0);
	const [objectDb, _updateObjectDb] = useState(game.objectDb());

	const doAction = (player, action) => {
		game.doAction(player, action);
		rerender(num + 1);
	};

	const gameWrapper = {
		game,
		objectDb,
		doAction,
	};

	return (
		<GameContext.Provider value={gameWrapper}>
			{children}
		</GameContext.Provider>
	);
}