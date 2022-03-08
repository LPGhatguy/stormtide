import { useContext } from "react"
import styled from "styled-components"

import { GameContext } from "./GameRoot"

const transforms = (props) => {
  let transforms = []

  if (props.translate === "down") {
    transforms.push("translateY(10%)")
  } else if (props.translate === "up") {
    transforms.push("translateY(-10%)")
  }

  if (props.tapped) {
    transforms.push("rotate(90deg)")
  }

  if (transforms.length === 0) {
    transforms.push("none")
  }

  return `transform: ${transforms.join(", ")};`
}

const StyledCard = styled.div`
  flex: 0 1 4rem;
  transition: transform 200ms ease-in-out;
  text-align: center;

  aspect-ratio: ${(props) => (props.canTap ? "1" : "5 / 7")};

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
`

const CardImage = styled.img`
  box-shadow: ${(props) =>
    props.redZone ? "0 0 8px 2px rgba(255, 0, 0, 0.8)" : ""};
`

export default function Card({
  id,
  canTap,
  tapped,
  onClick,
  redZone,
  translate,
}) {
  const { objectDb } = useContext(GameContext)
  const card = objectDb.card(id)

  let image
  if (card != null) {
    image = card.image
  }

  return (
    <StyledCard canTap={canTap} tapped={tapped} translate={translate}>
      <CardImage
        role="button"
        onClick={onClick}
        redZone={redZone}
        src={image}
      />
    </StyledCard>
  )
}
