use serde::{Deserialize, Serialize};

use crate::ident::Ident;

/// 205.4a A card can also have one or more supertypes. These are printed
///        directly before its card types. The supertypes are basic, legendary,
///        ongoing, snow, and world.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CardSupertype {
    Basic,
    Legendary,
    Ongoing,
    Snow,
    World,
}

/// 205.2a The card types are artifact, conspiracy, creature, enchantment,
///        instant, land, phenomenon, plane, planeswalker, scheme, sorcery,
///        tribal, and vanguard. See section 3, “Card Types.”
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CardType {
    Artifact,
    Conspiracy,
    Creature,
    Enchantment,
    Instant,
    Land,
    Phenomenon,
    Plane,
    Planeswalker,
    Scheme,
    Sorcery,
    Tribal,
    Vanguard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CardSubtype {
    pub name: Ident,
}
