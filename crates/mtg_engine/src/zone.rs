use hecs::Entity;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::player::PlayerId;

/// 400.1. A zone is a place where objects can be during a game. There are
///        normally seven zones: library, hand, battlefield, graveyard, stack,
///        exile, and command. Some older cards also use the ante zone. Each
///        player has their own library, hand, and graveyard. The other zones
///        are shared by all players.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ZoneId {
    Library(PlayerId),
    Hand(PlayerId),
    Graveyard(PlayerId),
    Stack,
    Battlefield,
    Exile,
    Command,
}

#[derive(Debug)]
pub struct Zone {
    members: Vec<Entity>,
}

impl Zone {
    pub(crate) fn new() -> Self {
        Self {
            members: Vec::new(),
        }
    }

    pub(crate) fn clear(&mut self) {
        self.members.clear();
    }

    pub(crate) fn add(&mut self, entity: Entity) {
        self.members.push(entity);
    }

    pub(crate) fn remove(&mut self, entity: Entity) {
        self.members.retain(|e| *e != entity);
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.members.shuffle(&mut rng);
    }

    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    pub fn members(&self) -> &[Entity] {
        &self.members
    }
}

#[derive(Debug)]
pub struct Zones {
    pub libraries: Vec<Zone>,
    pub hands: Vec<Zone>,
    pub graveyards: Vec<Zone>,
    pub stack: Zone,
    pub battlefield: Zone,
    pub exile: Zone,
    pub command: Zone,
}

impl Zones {
    pub fn new(num_players: usize) -> Self {
        let libraries = (0..num_players).map(|_| Zone::new()).collect();
        let hands = (0..num_players).map(|_| Zone::new()).collect();
        let graveyards = (0..num_players).map(|_| Zone::new()).collect();
        let stack = Zone::new();
        let battlefield = Zone::new();
        let exile = Zone::new();
        let command = Zone::new();

        Self {
            libraries,
            hands,
            graveyards,
            stack,
            battlefield,
            exile,
            command,
        }
    }
}
