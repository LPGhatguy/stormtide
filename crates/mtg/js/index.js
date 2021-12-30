import init, { Game } from "mtg";
import React from "react";
import { render } from "react-dom";
import styled from "styled-components";

import card_back from "../assets/card-back.png";
import player1 from "../assets/player1.png";
import player2 from "../assets/player2.png";

const SidePanel = styled.div`
	display: flex;
	flex: 0 0 10rem;
	flex-direction: column;
	background-color: #0f0f10;
	color: #fefefe;
`;

const Identity = styled.div`
	display: flex;
	align-items: center;
	gap: 1rem;
	padding: 1rem;
	border-bottom: 1px solid #fefefe;
`;

const Portrait = styled.div`
	display: flex;
	flex: 0 0 2rem;
	aspect-ratio: 1 / 1;

	img {
		width: 100%;
		height: 100%;
	}
`;

const Name = styled.div`
	flex: 1 1;
	font-size: 1.2rem;
	font-weight: bold;
	text-align: center;
`;

const LifeTotal = styled.div`
	font-size: 1.4rem;
	text-align: center;
`;

const Library = styled.div`
	margin: 1rem;
	aspect-ratio: 5 / 7;
	background-image: url(${ card_back });
	background-size: contain;

	display: flex;
	justify-content: center;
	align-items: center;
	font-weight: bold;
	-webkit-text-stroke: 2px black;
	color: white;
	font-size: 2rem;
`;

function PlayerPanel({ name, lifeTotal, libraryCount, profilePicture }) {
	return (
		<SidePanel>
			<Identity>
				<Portrait>
					<img src={ profilePicture } />
				</Portrait>
				<Name>{ name }</Name>
			</Identity>
			<LifeTotal>{ lifeTotal }</LifeTotal>
			<Library>{ libraryCount }</Library>
		</SidePanel>
	);
}


const GameContainer = styled.div`
	flex: 1 0;

	display: flex;
	flex-direction: column;
	gap: 1rem;
	background-color: black;
`;

const Player = styled.div`
	flex: 1 0;
	display: flex;
`;

const MainPlayfield = styled.div`
	flex: 1 0;

	background-color: #242526;
`;

const profilePictures = [player1, player2];

function App({ game }) {
	const players = game.players().map((player, index) => {
		const library = game.objectsInZone({ "Library": player.entity });

		return (
			<Player>
				<PlayerPanel
					name={player.name}
					lifeTotal={player.lifeTotal}
					libraryCount={library.length}
					profilePicture={profilePictures[index]} />
				<MainPlayfield />
			</Player>
		);
	});

	return (
		<GameContainer>
			{players}
		</GameContainer>
	);
}

async function main() {
	console.log("Loading WebAssembly...");
	await init();
	console.log("WebAssembly loaded!");

	let game = new Game();
	console.log(game.players());

	const root = document.getElementById("app");
	render(<App game={game} />, root);
}

main();