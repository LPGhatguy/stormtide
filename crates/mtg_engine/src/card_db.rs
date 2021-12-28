use std::collections::HashMap;

use crate::card::CardDescriptor;

static DATA_JSON: &str = include_str!("./card_db.json");

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CardId(usize);

pub struct CardDb {
    cards: Vec<CardDescriptor>,
    name_to_index: HashMap<String, usize>,
}

impl CardDb {
    pub fn load() -> Self {
        let cards: Vec<CardDescriptor> = serde_json::from_str(DATA_JSON).unwrap();

        let mut name_to_index = HashMap::new();
        for (index, card) in cards.iter().enumerate() {
            name_to_index.insert(card.name.clone(), index);
        }

        Self {
            cards,
            name_to_index,
        }
    }

    pub fn card_id(&self, name: &str) -> Option<CardId> {
        self.name_to_index.get(name).copied().map(CardId)
    }

    pub fn card(&self, id: CardId) -> Option<&CardDescriptor> {
        self.cards.get(id.0)
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
        let db = CardDb::load();
        println!("{:?}", db.cards);
    }
}
