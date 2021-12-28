use hecs::Entity;
use rand::seq::SliceRandom;

/// 400.1. A zone is a place where objects can be during a game. There are
///        normally seven zones: library, hand, battlefield, graveyard, stack,
///        exile, and command. Some older cards also use the ante zone. Each
///        player has their own library, hand, and graveyard. The other zones
///        are shared by all players.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ZoneId {
    Library(Entity),
    Hand(Entity),
    Graveyard(Entity),
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
