//! Contains structures for Power-Toughness and related effects.

use hecs::Entity;

/// Container for power and toughness, helping simplify calculations.
#[derive(Debug, Clone, Copy)]
pub struct PtValue {
    pub power: i64,
    pub toughness: i64,
}

/// Represent base power and toughness that can be present on a creature. Some
/// creatures have special rules that determine their base power and toughness,
/// which can be further affected by other effects.
#[derive(Debug, Clone, Copy)]
pub enum PtCharacteristic {
    Normal(PtValue),
}

impl PtCharacteristic {
    /// This method will need access to the game state to do things like:
    /// - count the number of card types in all graveyards (Tarmogoyf)
    /// - count the number of cards in its owner's hand (Maro)
    pub fn resolve(&self) -> PtValue {
        match self {
            Self::Normal(value) => *value,
        }
    }
}

/// An effect that sets the power/toughness of a creature directly, like Lignify
/// or March of the Machines.
///
/// Applies in layer 7B.
pub struct SetPtEffect {
    // TODO: Change to selector type
    pub target: Entity,

    // TODO: Can these effects be more complicated than just setting a value?
    pub value: PtValue,
}

/// An effect that modifies the power/toughness of a creature without directly
/// setting it, like Giant Growth or Shared Triumph.
///
/// Applies in layer 7C.
pub struct AdjustPtEffect {
    // TODO: Change to selector type
    pub target: Entity,

    // TODO: More complicated adjustments than just addition?
    pub adjustment: PtValue,
}

/// An effect that switches the power and toughness of a creature, like Inside
/// Out or Merfolk Thaumaturgist.
///
/// Applies in layer 7E.
pub struct SwitchPtEffect {
    // TODO: Change to selector type
    pub target: Entity,
}
