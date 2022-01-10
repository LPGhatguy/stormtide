use serde::{Deserialize, Serialize};

use crate::cost::ManaCost;
use crate::ident::Ident;
use crate::player::PlayerId;
use crate::pt::PtCharacteristic;
use crate::types::{CardSubtype, CardSupertype, CardType};
use crate::zone::ZoneId;

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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Object {
    pub name: Ident,
    pub types: Vec<CardType>,
    pub supertypes: Vec<CardSupertype>,
    pub subtypes: Vec<CardSubtype>,
    pub pt: Option<PtCharacteristic>,
    pub mana_cost: Option<ManaCost>,

    pub zone: ZoneId,

    pub owner: PlayerId,

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
    pub controller: Option<PlayerId>,
}
