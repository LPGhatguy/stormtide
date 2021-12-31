import init, { startLogging, Game } from "mtg";
import React, { useContext } from "react";
import { render } from "react-dom";
import styled from "styled-components";

import Steps from "./components/Steps";
import GameRoot, { GameContext } from "./components/GameRoot";
import Card from "./components/Card";
import PlayerPanel from "./components/PlayerPanel";

import card_back from "../assets/card-back.png";
import player1 from "../assets/player1.png";
import player2 from "../assets/player2.png";

const GameContainer = styled.div`
	flex: 1 0;

	display: flex;
	flex-direction: column;
`;

const Player = styled.div`
	flex: 1 0;
	display: flex;
`;

const MainPlayfield = styled.div`
	flex: 1 0;
	background-color: #242526;

	display: flex;
	gap: 0.5rem;
	flex-direction: ${props => props.top ? "column-reverse" : "column"};
`;

const BattlefieldRow = styled.div`
	flex: 1 1 auto;

	display: flex;
	justify-content: center;
	gap: 0.5rem;
`;

const Hand = styled.div`
	flex: 0 1 auto;
	display: flex;
	background-color: rgba(0, 0, 0, 0.5);
`;

const profilePictures = [player1, player2];

const getPriority = game => {
	const state = game.state();
	if (state.type === "Priority") {
		return state.player;
	} else {
		return null;
	}
};

function Main() {
	const { game } = useContext(GameContext);

	const priority = getPriority(game);
	const players = game.players().map((player, index) => {
		const library = game.objectsInZone({ "Library": player.entity });
		const top = index === 0;

		return (
			<Player>
				<PlayerPanel
					player={player.entity}
					name={player.name}
					priority={priority === player.entity}
					lifeTotal={player.life}
					libraryCount={library.length}
					profilePicture={profilePictures[index]} />

				<MainPlayfield top={top}>
					<BattlefieldRow>
						<Card id={1} /> 
						<Card id={1} /> 
					</BattlefieldRow>
					<BattlefieldRow>
						<Card id={0} />
						<Card id={0} />
					</BattlefieldRow>
					<Hand>
						<Card id={0} /> 
						<Card id={2} /> 
						<Card id={0} /> 
						<Card id={3} /> 
					</Hand>
				</MainPlayfield>
			</Player>
		);
	});

	return (
		<GameContainer>
			{players[0]}
			<Steps currentStep={game.step()} />
			{players[1]}
		</GameContainer>
	);
}

function App({ game }) {
	return (
		<GameRoot game={game}>
			<Main />
		</GameRoot>
	);
}

async function main() {
	console.log("Loading WebAssembly...");
	await init();
	console.log("WebAssembly loaded!");

	startLogging();

	let game = new Game();
	console.log("Players:", game.players());
	console.log("State:", game.state());

	const root = document.getElementById("app");
	render(<App game={game} />, root);
}

main();