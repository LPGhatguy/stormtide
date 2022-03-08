import styled from "styled-components"
import { useContext } from "react"

import { GameContext } from "./GameRoot"

const Container = styled.div`
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.2rem;
`

const Label = styled.span`
  font-size: 1.2rem;
`

const State = styled.span`
  font-size: 0.8rem;
  font-family: monospace;
`

export default function StateBanner() {
  const { game } = useContext(GameContext)

  return (
    <Container>
      <Label>Current Game State:</Label>
      <State>{JSON.stringify(game.state())}</State>
    </Container>
  )
}
