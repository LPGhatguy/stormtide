//! Qualities that make up a card

use serde::{Deserialize, Serialize};

use crate::cost::ManaCost;
use crate::ident::Ident;
use crate::pt::PtCharacteristic;
use crate::types::{CardSubtype, CardSupertype, CardType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardDescriptor {
    pub name: Ident,
    pub types: Vec<CardType>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supertypes: Vec<CardSupertype>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subtypes: Vec<CardSubtype>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mana_cost: Option<ManaCost>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pt: Option<PtCharacteristic>,
}
