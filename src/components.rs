//! Components used in the game's hecs `World` object.

use std::collections::BTreeSet;

use hecs::Entity;

use crate::pt::PtCharacteristic;

/// A component that indicates that this entity is a player.
#[derive(Debug)]
pub struct Player;

/// A component to indicate that this entity is an effect that should be cleaned
/// up at the end of the turn.
#[derive(Debug)]
pub struct UntilEotEffect;

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
pub struct Land;
