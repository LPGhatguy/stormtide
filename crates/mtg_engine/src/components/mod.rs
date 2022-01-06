//! Components used in the game's hecs `World` object.

use std::collections::BTreeSet;

use hecs::Entity;
use serde::{Deserialize, Serialize};

use crate::counters::Counter;
use crate::mana_pool::ManaId;
use crate::object_db::CardId;
use crate::pt::PtCharacteristic;

mod object;
mod player;

pub use object::*;
pub use player::*;

/// A component to indicate that this entity is an effect that should be cleaned
/// up at the end of the turn.
#[derive(Debug)]
pub struct UntilEotEffect;

/// Indicates that this entity's lifetime is tied to another entity. Used for
/// clearing static effects attached to permanents.
#[derive(Debug)]
pub struct AttachedToEntity {
    pub target: Entity,
}

/// 108.2. When a rule or text on a card refers to a “card,” it means only a
///        Magic card or an object represented by a Magic card.
///
/// 108.2a Most Magic games use only traditional Magic cards, which measure
///        approximately 2.5 inches (6.3 cm) by 3.5 inches (8.8 cm). Certain
///        formats also use nontraditional Magic cards, oversized cards that may
///        have different backs.
///
/// 108.2b Tokens aren’t considered cards—even a card-sized game supplement that
///        represents a token isn’t considered a card for rules purposes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: CardId,
}

#[derive(Debug)]
pub struct IncompleteSpell {
    pub cost_paid: Vec<ManaId>,
}

#[derive(Debug)]
pub struct Spell {
    pub targets: BTreeSet<Entity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permanent {
    pub tapped: bool,
}

#[derive(Debug)]
pub struct Creature {
    pub pt: PtCharacteristic,
}

#[derive(Debug)]
pub struct Counters {
    pub counters: Vec<Counter>,
}

#[derive(Debug)]
pub struct Damage {
    pub amount: u64,
}

#[derive(Debug)]
pub struct Land;
