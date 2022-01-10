use hecs::Entity;
use serde::{Deserialize, Serialize};

use crate::player::PlayerId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Target {
    Object(Entity),
    Player(PlayerId),
}
