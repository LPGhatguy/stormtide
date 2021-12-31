import { useContext } from "react";
import styled from "styled-components";

import { GameContext } from "./GameRoot";

const transforms = props => {
	let transforms = [];

	if (props.translate === "down") {
		transforms.push("translateY(10%)");
	} else if (props.translate === "up") {
		transforms.push("translateY(-10%)");
	}

	if (props.tapped) {
		transforms.push("rotate(90deg)");
	}

	if (transforms.length === 0) {
		transforms.push("none");
	}

	return `transform: ${transforms.join(", ")};`
};

const StyledCard = styled.div`
	flex: 0 1 4rem;
	aspect-ratio: 1;
	transition: transform 200ms ease-in-out;
	text-align: center;

	${transforms}

	a {
		display: inline-block;
		height: 100%;
	}

	img {
		height: 100%;
		aspect-ratio: 5 / 7;
		cursor: pointer;
	}
`;

export default function Card({ id, onClick }) {
	const { objectDb } = useContext(GameContext);
	const card = objectDb.card(id);

	let image;
	if (card != null) {
		image = card.image;
	}

	return (
		<StyledCard>
			<img role="button" onClick={onClick} src={ image } />
		</StyledCard>
	);
}