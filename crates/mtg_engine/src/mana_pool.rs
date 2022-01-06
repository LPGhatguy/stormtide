use thunderdome::{Arena, Index};

use crate::cost::ManaColor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ManaId(Index);

#[derive(Debug)]
pub struct ManaPool {
    manas: Arena<Mana>,
}

impl ManaPool {
    pub fn new() -> Self {
        Self {
            manas: Arena::new(),
        }
    }

    pub fn clear(&mut self) {
        self.manas.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.manas.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (ManaId, Mana)> + '_ {
        self.manas
            .iter()
            .map(|(index, &mana)| (ManaId(index), mana))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Mana {
    color: ManaColor,
}
