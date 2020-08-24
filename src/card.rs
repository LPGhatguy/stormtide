//! Qualities that make up a card

use crate::cost::Cost;

pub struct CardDescriptor {
    name: String,
    primary_cost: Cost,
}
