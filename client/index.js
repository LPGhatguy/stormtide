import init, { Game } from "mtg"
import React, { useContext } from "react"
import { render } from "react-dom"
import styled from "styled-components"

import Steps from "./components/Steps"
import GameRoot, { GameContext } from "./components/GameRoot"
import Card from "./components/Card"
import PlayerPanel from "./components/PlayerPanel"
import StackWindow from "./components/StackWindow"
import StateBanner from "./components/StateBanner"
import CombatState, { CombatContext } from "./components/CombatState"

import card_back from "~/assets/card-back.png"
import player1 from "~/assets/player1.png"
import player2 from "~/assets/player2.png"

const GameContainer = styled.div`
  flex: 1 0;

  display: flex;
  flex-direction: column;
`

const Player = styled.div`
  flex: 1 0;
  display: flex;
`

const MainPlayfield = styled.div`
  flex: 1 0;
  background-color: #242526;

  display: flex;
  gap: 0.5rem;
  flex-direction: ${(props) => (props.top ? "column-reverse" : "column")};
  ${(props) => (props.top ? "padding-bottom" : "padding-top")}: 0.5rem;
`

const BattlefieldRow = styled.div`
  flex: 1 1 2rem;

  display: flex;
  justify-content: center;
`

const Hand = styled.div`
  flex: 1 1 1rem;

  display: flex;
  background-color: rgba(0, 0, 0, 0.5);
`

const profilePictures = [player1, player2]

const getPriority = (game) => {
  const state = game.state()
  if (state.type === "Player" && state.action === "Priority") {
    return state.player
  } else {
    return null
  }
}

function Main() {
  const { game, doAction } = useContext(GameContext)
  const combat = useContext(CombatContext)
  const state = game.state()

  const priority = getPriority(game)
  const battlefield = game.objectsInZone("Battlefield")
  const players = game.players().map((player, index) => {
    const hand = game.objectsInZone({ Hand: player.id })
    const library = game.objectsInZone({ Library: player.id })
    const top = index === 0

    const playCard = (object) => {
      if (object.types.includes("Land")) {
        doAction(player.id, {
          type: "PlayLand",
          card: object.entity,
        })
      } else if (object.types.includes("Creature")) {
        doAction(player.id, {
          type: "StartCastingSpell",
          spell: object.entity,
        })
      } else {
        console.warn("Can't play this object yet:", object)
      }
    }

    const handCards = hand.map((object) => {
      const id = object.card ? object.card.id : null

      return (
        <Card key={object.entity} id={id} onClick={() => playCard(object)} />
      )
    })

    const creatureCards = battlefield
      .filter(
        (object) =>
          object.controller === player.id && !object.types.includes("Land")
      )
      .map((object) => {
        let tapped = false
        if (object.permanent != null && object.permanent.tapped) {
          tapped = true
        }

        const id = object.card ? object.card.id : null

        let redZone = false
        let translate = null
        let onClick = null

        if (
          state.type === "Player" &&
          state.action === "ChooseAttackers" &&
          state.player === player.id
        ) {
          console.log("Creature is eligible for combat:", object.entity)
          if (combat.attackers.includes(object.entity)) {
            redZone = true
            translate = top ? "down" : "up"
          }

          onClick = () => {
            console.log("Creature clicked!")

            if (combat.attackers.includes(object.entity)) {
              combat.setAttackers(
                combat.attackers.filter((x) => x !== object.entity)
              )
            } else {
              combat.setAttackers([...combat.attackers, object.entity])
            }
          }
        }

        return (
          <Card
            key={object.entity}
            canTap={true}
            tapped={tapped}
            id={id}
            translate={translate}
            redZone={redZone}
            onClick={onClick}
          />
        )
      })

    const manaCards = battlefield
      .filter(
        (object) =>
          object.controller === player.id && object.types.includes("Land")
      )
      .map((object) => {
        let tapped = false
        if (object.permanent != null && object.permanent.tapped) {
          tapped = true
        }

        const id = object.card ? object.card.id : null

        return <Card key={object.entity} canTap={true} id={id} />
      })

    return (
      <Player>
        <PlayerPanel
          player={player}
          priority={priority === player.id}
          libraryCount={library.length}
          profilePicture={profilePictures[index]}
        />

        <MainPlayfield top={top}>
          <BattlefieldRow>{creatureCards}</BattlefieldRow>
          <BattlefieldRow>{manaCards}</BattlefieldRow>
          <Hand>{handCards}</Hand>
        </MainPlayfield>
      </Player>
    )
  })

  const stack = game.objectsInZone("Stack")

  return (
    <GameContainer>
      <StateBanner />
      {players[0]}
      <Steps currentStep={game.step()} />
      {players[1]}
      <StackWindow objects={stack} />
    </GameContainer>
  )
}

function App({ game }) {
  return (
    <GameRoot game={game}>
      <CombatState>
        <Main />
      </CombatState>
    </GameRoot>
  )
}

async function main() {
  console.log("Loading WebAssembly...")
  await init()
  console.log("WebAssembly loaded!")

  let game = new Game()
  console.log("Players:", game.players())
  console.log("State:", game.state())

  const root = document.getElementById("app")
  render(<App game={game} />, root)
}

main()
