//! Defines the high-level structure describing a game of Magic.

use std::collections::HashMap;

use hecs::{Entity, World};

use crate::action::Action;
use crate::components::Player;
use crate::queries::Query;

pub struct Game {
    pub world: World,
    pub turn_order: Vec<Entity>,
    active_player: Entity,
    priority: Option<Entity>,
    step: Step,
    zones: HashMap<ZoneId, Zone>,
}

impl Game {
    pub fn new() -> Self {
        let mut world = World::new();

        let player1 = world.spawn((Player,));
        let player2 = world.spawn((Player,));

        Self {
            world,
            turn_order: vec![player1, player2],
            active_player: player1,
            priority: Some(player1),
            step: Step::Upkeep,
            zones: HashMap::new(),
        }
    }

    /// Resolve a given query to compute a property of the game state, like a
    /// property of a game object.
    pub fn query<Q: Query>(&self, query_object: Q) -> Q::Output {
        query_object.query(&self.world)
    }

    pub fn possible_actions(&self, player: Entity) -> Vec<Action> {
        let mut actions = vec![Action::Concede];

        if self.priority == Some(player) {
            actions.push(Action::PassPriority);
        }

        actions
    }

    pub fn do_action(&mut self, player: Entity, action: Action) {
        match action {
            Action::Concede => unimplemented!("complete game"),
            Action::PassPriority => self.pass_priority(player),
            Action::CastSpell { spell } => {
                unimplemented!("player {:?} casting spell {:?}", player, spell)
            }
            Action::ActivateAbility { object, ability } => unimplemented!(
                "player {:?} activating ability #{} on object {:?}",
                player,
                ability,
                object
            ),
            Action::PlayLand { card } => {
                unimplemented!("player {:?} playing land {:?}", player, card)
            }
        }
    }

    fn apply_state_based_actions(&mut self) {}

    fn pass_priority(&mut self, player: Entity) {
        if self.priority != Some(player) {
            return;
        }
    }
}

#[derive(Debug)]
pub enum Step {
    // Beginning phases
    Untap,
    Upkeep,
    Draw,

    // First main phase
    Main1,

    // Combat phase
    BeginCombat,
    DeclareAttacks,
    DeclareBlocks,
    Damage,

    // Second main phase
    Main2,

    // End phase
    EndStep,
    CleanupStep,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ZoneId {
    Library(Entity),
    Hand(Entity),
    Graveyard(Entity),
    Stack,
    Battlefield,
    Exile,
}

#[derive(Debug)]
struct Zone {
    members: Vec<Entity>,
}

impl Zone {
    fn new() -> Self {
        Self {
            members: Vec::new(),
        }
    }
}
