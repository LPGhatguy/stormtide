import { useContext } from "react"
import styled from "styled-components"

import { GameContext } from "./GameRoot"

const DebugButton = styled.button`
  width: 100%;
  padding: 0.2rem 0.5rem;
  margin: 0.2rem;
`

export default function DebugActions({ player }) {
  const { game, doAction } = useContext(GameContext)

  const concede = () => doAction(player, { type: "Concede" })
  const actions = [
    <DebugButton key="concede" onClick={concede}>
      Concede
    </DebugButton>,
  ]

  const state = game.state()

  if (state.type !== "Player") {
    return <div></div>
  }

  if (state.player === player) {
    switch (state.action) {
      case "Priority":
        const passPriority = () => doAction(player, { type: "PassPriority" })
        actions.push(
          <DebugButton key="pass" onClick={passPriority}>
            Pass Priority
          </DebugButton>
        )
        break

      case "ChooseAttackers":
        const noAttackers = () =>
          doAction(player, { type: "ChooseAttackers", attackers: [] })
        actions.push(
          <DebugButton key="no-attackers" onClick={noAttackers}>
            No Attackers
          </DebugButton>
        )
        break

      case "ChooseBlockers":
        const noBlockers = () =>
          doAction(player, { type: "ChooseBlockers", blockers: [] })
        actions.push(
          <DebugButton key="no-blockers" onClick={noBlockers}>
            No Blockers
          </DebugButton>
        )
        break
    }
  }

  return <div>{actions}</div>
}
