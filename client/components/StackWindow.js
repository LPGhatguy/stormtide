import styled from "styled-components"

import Card from "./Card"

const StyledStackWindow = styled.div`
  display: flex;
  flex-direction: column;
  background-color: rgba(0, 0, 0, 0.5);
`

const StackTitle = styled.div`
  flex: 0 0 auto;
  padding: 0.2rem;
  font-size: 1.4rem;
  border-bottom: 1px solid black;
`

const StackBody = styled.div`
  flex: 1 0 8rem;
  padding: 0.2rem;
  display: flex;
`

export default function StackWindow({ objects }) {
  if (objects.length === 0) {
    return null
  }

  const cards = objects.map((object) => {
    const id = object.card ? object.card.id : null

    return <Card key={object.entity} id={id} />
  })

  return (
    <StyledStackWindow>
      <StackTitle>The Stack</StackTitle>
      <StackBody>{cards}</StackBody>
    </StyledStackWindow>
  )
}
