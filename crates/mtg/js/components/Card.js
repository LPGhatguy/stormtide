import { useContext } from "react";
import styled from "styled-components";

import { GameContext } from "./GameRoot";

const StyledCard = styled.div`
	flex: 0 1 4rem;
	aspect-ratio: 5 / 7;

	> img {
		width: 100%;
	}
`;

export default function Card({ id }) {
	const { objectDb } = useContext(GameContext);
	const card = objectDb.card(id);

	let image;
	if (card != null) {
		image = card.image;
	}

	return (
		<StyledCard>
			<img src={ image } />
		</StyledCard>
	);
}