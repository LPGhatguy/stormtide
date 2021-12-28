//! Qualities that make up a card

use serde::{Deserialize, Serialize};

use crate::cost::Cost;
use crate::types::{CardSubtype, CardSupertype, CardType};

#[derive(Debug, Serialize, Deserialize)]
pub struct CardDescriptor {
    pub name: String,
    pub types: Vec<CardType>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supertypes: Vec<CardSupertype>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subtypes: Vec<CardSubtype>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_cost: Option<Cost>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}
