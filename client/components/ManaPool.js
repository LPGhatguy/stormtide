import styled from "styled-components"

import mana1 from "~/assets/mana-1.svg"
import manaW from "~/assets/mana-w.svg"
import manaU from "~/assets/mana-u.svg"
import manaB from "~/assets/mana-b.svg"
import manaR from "~/assets/mana-r.svg"
import manaG from "~/assets/mana-g.svg"

const symbolMap = {
  White: manaW,
  Blue: manaU,
  Black: manaB,
  Red: manaR,
  Green: manaG,
}

const StyledManaPool = styled.div`
  display: flex;
  flex-wrap: wrap;
  gap: 0.25rem;
  padding: 0.25rem;
`

const ManaSymbol = styled.img`
  width: 1.2rem;
`

const Mana = ({ mana, onClick }) => {
  const symbol = symbolMap[mana.color]

  return (
    <a href="#" onClick={onClick}>
      <ManaSymbol src={symbol} />
    </a>
  )
}

export default function ManaPool({ pool, onClick }) {
  return (
    <StyledManaPool>
      {pool.map((mana, i) => (
        <Mana key={i} mana={mana} onClick={() => onClick && onClick(i)} />
      ))}
    </StyledManaPool>
  )
}
