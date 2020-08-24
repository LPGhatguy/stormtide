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
