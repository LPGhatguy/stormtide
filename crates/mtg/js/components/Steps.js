import React from "react"
import styled from "styled-components"

const stepIds = [
  "Untap",
  "Upkeep",
  "Draw",
  "Main1",
  "BeginCombat",
  "DeclareAttackers",
  "DeclareBlockers",
  "CombatDamage",
  "EndCombat",
  "Main2",
  "End",
  "Cleanup",
]

const stepLabelOverrides = {
  Main1: "Main",
  Main2: "Main",
  BeginCombat: "Begin",
  DeclareAttackers: "Attackers",
  DeclareBlockers: "Blockers",
  CombatDamage: "Damage",
  EndCombat: "End",
}

const phases = [
  {
    name: "Beginning",
    steps: ["Untap", "Upkeep", "Draw"],
  },
  {
    name: "Main 1",
    steps: ["Main1"],
  },
  {
    name: "Combat",
    steps: [
      "BeginCombat",
      "DeclareAttackers",
      "DeclareBlockers",
      "CombatDamage",
      "EndCombat",
    ],
  },
  {
    name: "Main 2",
    steps: ["Main2"],
  },
  {
    name: "Ending",
    steps: ["End", "Cleanup"],
  },
]

const StyledSteps = styled.div`
  display: flex;
  justify-content: center;
  gap: 0.5rem;

  background-color: rgba(0, 0, 0, 0.2);
  padding: 0.2rem;
  text-align: center;
`

const StyledPhase = styled.div`
  --accent-color: ${(props) => (props.active ? "#fefefe" : "#aeaeae")};
  color: var(--accent-color);

  flex: 0 0 auto;
  margin: 0 0.5rem;
  text-align: center;
`

const PhaseUnderBox = styled.div`
  display: flex;
  align-items: center;
`

const PhaseUnderline = styled.div`
  flex: 1 0 0.5rem;
  min-width: 0.5rem; // needed due to mysterious flexbox behavior
  background-color: var(--accent-color);
  height: 0.5px;

  position: relative;
  &::before {
    content: "";
    position: absolute;
    bottom: 0;
    // using width over 1px causes borders to be sometimes >1px thick when
    // DPI scaling gets involved.
    width: 0.5px;
    height: 8px;
    background-color: var(--accent-color);
  }

  &:first-child::before {
    right: 100%;
  }

  &:last-child::before {
    left: 100%;
  }
`

const PhaseLabel = styled.div`
  padding: 0 0.5em;
  flex: 0 0 auto;
  font-size: 0.65rem;
`

const PhaseSteps = styled.div`
  display: flex;
  justify-content: center;
`

const Step = styled.div`
  padding: 0.15rem 0.5rem;
  color: ${(props) => (props.active ? "#fefefe" : "#aeaeae")};
`

function Phase({ name, currentStep, steps }) {
  const active = steps.includes(currentStep)
  const children = steps.map((step) => (
    <Step key={step} active={currentStep === step}>
      {stepLabelOverrides[step] || step}
    </Step>
  ))

  return (
    <StyledPhase active={active}>
      <PhaseSteps>{children}</PhaseSteps>
      <PhaseUnderBox>
        <PhaseUnderline />
        <PhaseLabel>{name}</PhaseLabel>
        <PhaseUnderline />
      </PhaseUnderBox>
    </StyledPhase>
  )
}

export default function Steps({ currentStep }) {
  const children = phases.map((phase) => (
    <Phase
      key={phase.name}
      name={phase.name}
      currentStep={currentStep}
      steps={phase.steps}
    />
  ))

  return <StyledSteps>{children}</StyledSteps>
}
