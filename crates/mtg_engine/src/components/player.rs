use serde::{Deserialize, Serialize};

use crate::mana_pool::ManaPool;

/// A component that indicates that this entity is a player.
#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub has_lost: bool,
    pub lands_played_this_turn: u32,
    pub life: i64,
    pub mana_pool: ManaPool,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,
            has_lost: false,
            lands_played_this_turn: 0,
            life: 20,
            mana_pool: ManaPool::new(),
        }
    }
}
