//! Components used in the game's hecs `World` object.

use std::collections::BTreeSet;

use hecs::Entity;

use crate::counters::Counter;
use crate::ident::Ident;
use crate::pt::PtCharacteristic;
use crate::zone::ZoneId;

/// A component that indicates that this entity is a player.
#[derive(Debug)]
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
///
/// 109.3. An object’s characteristics are name, mana cost, color, color
///        indicator, card type, subtype, supertype, rules text, abilities,
///        power, toughness, loyalty, hand modifier, and life modifier. Objects
///        can have some or all of these characteristics. Any other information
///        about an object isn’t a characteristic. For example, characteristics
///        don’t include whether a permanent is tapped, a spell’s target, an
///        object’s owner or controller, what an Aura enchants, and so on.
#[derive(Debug)]
pub struct Object {
    pub name: Ident,

    pub zone: ZoneId,

    pub owner: Entity,

    /// 109.4. Only objects on the stack or on the battlefield have a
    ///        controller. Objects that are neither on the stack nor on the
    ///        battlefield aren’t controlled by any player. See rule 108.4.
    ///        There are six exceptions to this rule:
    ///
    /// 109.4a The controller of a mana ability is determined as though it were
    ///        on the stack. See rule 605, “Mana Abilities.”
    ///
    /// 109.4b A triggered ability that has triggered but is waiting to be
    ///        placed on the stack is controlled by the player who controlled
    ///        its source at the time it triggered, unless it’s a delayed
    ///        triggered ability. To determine the controller of a delayed
    ///        triggered ability, see rules 603.7d–f. See also rule 603,
    ///        “Handling Triggered Abilities.”
    ///
    /// 109.4c An emblem is controlled by the player who puts it into the
    ///        command zone. See rule 114, “Emblems.”
    ///
    /// 109.4d In a Planechase game, a face-up plane or phenomenon card is
    ///        controlled by the player designated as the planar controller.
    ///        This is usually the active player. See rule 901.6.
    ///
    /// 109.4e In a Vanguard game, each vanguard card is controlled by its
    ///        owner. See rule 902.6.
    ///
    /// 109.4f In an Archenemy game, each scheme card is controlled by its
    ///        owner. See rule 904.7.
    ///
    /// 109.4g In a Conspiracy Draft game, each conspiracy card is controlled by
    ///        its owner. See rule 905.5.
    pub controller: Option<Entity>,
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
