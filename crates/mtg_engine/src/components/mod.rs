//! Components used in the game's hecs `World` object.

use std::collections::BTreeSet;

use hecs::Entity;
use serde::{Deserialize, Serialize};

use crate::cost::ManaCost;
use crate::counters::Counter;
use crate::mana_pool::ManaId;
use crate::object_db::CardId;
use crate::target::Target;
use crate::zone::ZoneId;

mod object;

pub use object::*;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncompleteSpell {
    pub previous_zone: ZoneId,
    pub total_cost: ManaCost,
    pub mana_paid: Vec<ManaId>,
    pub targets: Vec<Target>,
}

impl IncompleteSpell {
    pub fn new(previous_zone: ZoneId, total_cost: ManaCost) -> Self {
        Self {
            previous_zone,
            total_cost,
            mana_paid: Vec::new(),
            targets: Vec::new(),
        }
    }
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
pub struct Counters {
    pub counters: Vec<Counter>,
}

#[derive(Debug)]
pub struct Damage {
    pub amount: u64,
}
