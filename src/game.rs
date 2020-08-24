//! Defines the high-level structure describing a game of Magic.

use std::collections::HashMap;

use hecs::{Entity, World};

use crate::components::Player;
use crate::queries::Query;

pub struct Game {
    pub world: World,
    pub base_player_turn_order: Vec<Entity>,
    turn: Entity,
    priority: Entity,
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
            base_player_turn_order: vec![player1, player2],
            turn: player1,
            priority: player1,
            step: Step::Upkeep,
            zones: HashMap::new(),
        }
    }

    pub fn query<Q: Query>(&self, query_object: Q) -> Q::Output {
        query_object.query(&self.world)
    }

    fn pass_priority(&mut self) {}
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
