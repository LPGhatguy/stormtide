import { useContext } from "react"
import styled from "styled-components"

import { GameContext } from "./GameRoot"
import { CombatContext } from "./CombatState"

const DebugButton = styled.button`
  width: 100%;
  padding: 0.2rem 0.5rem;
  margin: 0.2rem;
`

export default function DebugActions({ player }) {
  const { game, doAction } = useContext(GameContext)
  const combat = useContext(CombatContext)

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
        const confirmAttackers = () => {
          doAction(player, {
            type: "ChooseAttackers",
            attackers: combat.attackers,
          })
          combat.setAttackers([])
        }
        actions.push(
          <DebugButton key="choose-attackers" onClick={confirmAttackers}>
            Confirm Attackers
          </DebugButton>
        )
        break

      case "ChooseBlockers":
        const confirmBlockers = () => {
          doAction(player, {
            type: "ChooseBlockers",
            blockers: combat.blockers,
          })
          combat.setBlockers([])
        }
        actions.push(
          <DebugButton key="choose-blockers" onClick={confirmBlockers}>
            Confirm Blockers
          </DebugButton>
        )
        break

      case "SpellManaAbilities":
        const spell = game
          .objectsInZone("Stack")
          .find(
            (object) =>
              object.controller === player && object.incompleteSpell != null
          )

        if (spell != null) {
          // TODO: Only show Finish Casting if the spell says it's done having
          // mana spent.
          const finishCasting = () =>
            doAction(player, {
              type: "FinishCastingSpell",
              spell: spell.entity,
            })

          actions.push(
            <DebugButton key="finish-casting" onClick={finishCasting}>
              Finish Casting
            </DebugButton>
          )

          const cancelCasting = () =>
            doAction(player, {
              type: "CancelCastingSpell",
              spell: spell.entity,
            })

          actions.push(
            <DebugButton key="cancel-casting" onClick={cancelCasting}>
              Cancel Spell
            </DebugButton>
          )
        }
        break
    }
  }

  return <div>{actions}</div>
}
