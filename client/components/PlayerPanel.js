import styled from "styled-components"
import { useContext } from "react"

import DebugActions from "./DebugActions"
import ManaPool from "./ManaPool"
import { GameContext } from "./GameRoot"

import card_back from "~/assets/card-back.png"

const SidePanel = styled.div`
  background-color: ${(props) => (props.priority ? "#44444b" : "#0f0f10")};
  display: flex;
  flex: 0 0 10rem;
  flex-direction: column;
  color: #fefefe;
`

const Identity = styled.div`
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1rem;
  border-bottom: 1px solid #fefefe;
`

const Portrait = styled.div`
  display: flex;
  flex: 0 0 2rem;
  aspect-ratio: 1 / 1;

  img {
    width: 100%;
    height: 100%;
  }
`

const Name = styled.div`
  flex: 1 1;
  font-size: 1.2rem;
  font-weight: bold;
  text-align: center;
`

const LifeTotal = styled.div`
  font-size: 1.4rem;
  text-align: center;
`

const Library = styled.div`
  margin: 1rem 3rem;
  aspect-ratio: 5 / 7;
  background-image: url(${card_back});
  background-size: contain;

  display: flex;
  justify-content: center;
  align-items: center;
  font-weight: bold;
  -webkit-text-stroke: 2px black;
  color: white;
  font-size: 2rem;
`

export default function PlayerPanel({
  player,
  priority,
  libraryCount,
  profilePicture,
}) {
  const { game, doAction } = useContext(GameContext)
  const clickMana = (i) => {
    const incompleteSpell = game
      .objectsInZone("Stack")
      .find(
        (object) =>
          object.controller === player.id && object.incompleteSpell != null
      )

    if (incompleteSpell == null) {
      return
    }

    doAction(player.id, {
      type: "PayIncompleteSpellMana",
      spell: incompleteSpell.entity,
      mana: i,
    })
  }

  return (
    <SidePanel priority={priority}>
      <Identity>
        <Portrait>
          <img src={profilePicture} />
        </Portrait>
        <Name>{player.name}</Name>
      </Identity>
      <LifeTotal>{player.lifeTotal}</LifeTotal>
      <Library>{libraryCount}</Library>
      <ManaPool pool={player.manaPool} onClick={clickMana} />
      <DebugActions player={player.id} />
    </SidePanel>
  )
}
