use serde::{Deserialize, Serialize};

use crate::cost::ManaColor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ManaId(u32);

#[derive(Debug, Serialize, Deserialize)]
pub struct ManaPool {
    manas: Vec<Mana>,
}

impl ManaPool {
    pub fn new() -> Self {
        Self { manas: Vec::new() }
    }

    pub fn get(&self, id: ManaId) -> Option<Mana> {
        self.manas.get(id.0 as usize).cloned()
    }

    pub fn clear(&mut self) {
        self.manas.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.manas.is_empty()
    }

    pub fn spend(&mut self, manas: &[ManaId]) {
        let mut index = 0;
        self.manas.retain(|_| {
            index += 1;
            !manas.contains(&ManaId(index - 1))
        });
    }

    pub fn iter(&self) -> impl Iterator<Item = (ManaId, Mana)> + '_ {
        self.manas
            .iter()
            .enumerate()
            .map(|(index, mana)| (ManaId(index as u32), *mana))
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Mana {
    pub color: ManaColor,
}
