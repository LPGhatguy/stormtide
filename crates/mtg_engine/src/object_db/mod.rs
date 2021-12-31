use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::card::CardDescriptor;
use crate::ident::Ident;

static CARDS_JSON: &str = include_str!("./cards.json");

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CardId(pub u32);

#[derive(Clone)]
pub struct ObjectDb {
    cards: Vec<CardDescriptor>,
    card_name_to_index: HashMap<Ident, usize>,
}

impl ObjectDb {
    pub fn load() -> Self {
        let cards: Vec<CardDescriptor> = serde_json::from_str(CARDS_JSON).unwrap();

        let mut card_name_to_index = HashMap::new();
        for (index, card) in cards.iter().enumerate() {
            card_name_to_index.insert(card.name.clone(), index);
        }

        Self {
            cards,
            card_name_to_index,
        }
    }

    pub fn card_id(&self, name: &str) -> Option<CardId> {
        self.card_name_to_index
            .get(name)
            .map(|&index| CardId(index as u32))
    }

    pub fn card(&self, id: CardId) -> Option<&CardDescriptor> {
        self.cards.get(id.0 as usize)
    }

    pub fn card_by_name(&self, name: &str) -> Option<&CardDescriptor> {
        let id = self.card_id(name)?;
        self.card(id)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn loads() {
        let db = ObjectDb::load();
        println!("{:?}", db.cards);
    }
}
