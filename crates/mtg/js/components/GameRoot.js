import React, { useState } from "react";

export const GameContext = React.createContext(null);

export default function GameRoot({ game, children }) {
	const [num, rerender] = useState(0);

	const doAction = (player, action) => {
		game.doAction(player, action);
		rerender(num + 1);
	};

	const gameWrapper = {
		game,
		doAction,
	};

	return (
		<GameContext.Provider value={gameWrapper}>
			{children}
		</GameContext.Provider>
	);
}