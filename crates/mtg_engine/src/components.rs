//! Components used in the game's hecs `World` object.

use std::collections::BTreeSet;

use hecs::Entity;

use crate::counters::Counter;
use crate::pt::PtCharacteristic;

/// A component that indicates that this entity is a player.
#[derive(Debug)]
pub struct Player {
    pub has_lost: bool,
    pub life: i64,
}

impl Player {
    pub fn new() -> Self {
        Self {
            has_lost: false,
            life: 20,
        }
    }
}

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

/// 109.1. An object is an ability on the stack, a card, a copy of a card, a
///        token, a spell, a permanent, or an emblem.
#[derive(Debug)]
pub struct Object {
    pub owner: Entity,
    pub controller: Entity,
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
#[derive(Debug)]
pub struct Card {}

#[derive(Debug)]
pub struct Spell {
    pub targets: BTreeSet<Entity>,
}

#[derive(Debug)]
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
