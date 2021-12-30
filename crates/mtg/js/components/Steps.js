import React from "react";
import styled from "styled-components";

const stepIds = [
	"Untap",
	"Upkeep",
	"Draw",
	"Main1",
	"BeginCombat",
	"DeclareAttackers",
	"DeclareBlockers",
	"CombatDamage",
	"EndOfCombat",
	"Main2",
	"End",
	"Cleanup"
];

const stepLabelOverrides = {
	"Main1": "Main #1",
	"Main2": "Main #2",
}

const StyledSteps = styled.div`
	display: flex;
	justify-content: center;
	gap: 0.5rem;

	background-color: rgba(0, 0, 0, 0.2);
	padding: 0.5rem;
	text-align: center;
`;

const StyledPhase = styled.div`
	flex: 0 0 auto;
	margin: 0 0.5rem;
	text-align: center;
`;

const PhaseUnderBox = styled.div`
	display: flex;
	align-items: center;
`;

const PhaseUnderline = styled.div`
	flex: 1 0 0.5rem;
	min-width: 0.5rem; // why do I need this?
	background-color: #fefefe;
	height: 1px;

	position: relative;
	&::before {
		content: "";
		position: absolute;
		bottom: 0;
		width: 1px;
		height: 8px;
		background-color: #fefefe;
	}

	&:first-child::before {
		right: 100%;
	}

	&:last-child::before {
		left: 100%;
	}
`;

const PhaseLabel = styled.div`
	padding: 0 0.5em;
	flex: 0 0 auto;
	font-size: 0.65rem;
`;

const PhaseSteps = styled.div`
	display: flex;
	justify-content: center;
`;

const Step = styled.div`
	padding: 0.15rem 0.5rem;
`;

const Phase = ({ name, children }) => (
	<StyledPhase>
		<PhaseSteps>
			{children}
		</PhaseSteps>
		<PhaseUnderBox>
			<PhaseUnderline />
			<PhaseLabel>{ name }</PhaseLabel>
			<PhaseUnderline />
		</PhaseUnderBox>
	</StyledPhase>
);

export default function Steps({ currentStep }) {
	return (
		<StyledSteps>
			<Phase name="Beginning">
				<Step>Untap</Step>
				<Step>Upkeep</Step>
				<Step>Draw</Step>
			</Phase>

			<Phase name="Main 1">
				<Step>Main</Step>
			</Phase>

			<Phase name="Combat">
				<Step>BeginCombat</Step>
				<Step>DeclareAttackers</Step>
				<Step>DeclareBlockers</Step>
				<Step>CombatDamage</Step>
				<Step>EndOfCombat</Step>
			</Phase>

			<Phase name="Main 2">
				<Step>Main</Step>
			</Phase>

			<Phase name="Ending">
				<Step>End</Step>
				<Step>Cleanup</Step>
			</Phase>
		</StyledSteps>
	);
}