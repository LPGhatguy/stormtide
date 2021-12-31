use serde::{Deserialize, Serialize};

/// A component that indicates that this entity is a player.
#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub has_lost: bool,
    pub life: i64,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,
            has_lost: false,
            life: 20,
        }
    }
}
