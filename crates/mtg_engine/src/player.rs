use serde::{Deserialize, Serialize};

use crate::cost::ManaColor;
use crate::mana_pool::{Mana, ManaPool};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlayerId(u32);

impl PlayerId {
    pub fn to_u32(&self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub has_lost: bool,
    pub lands_played_this_turn: u32,
    pub life: i64,
    pub mana_pool: ManaPool,
}

impl Player {
    pub fn new(id: PlayerId, name: String) -> Self {
        let mut mana_pool = ManaPool::new();
        mana_pool.add(Mana {
            color: ManaColor::Green,
        });
        mana_pool.add(Mana {
            color: ManaColor::Red,
        });
        mana_pool.add(Mana {
            color: ManaColor::Green,
        });

        Self {
            id,
            name,
            has_lost: false,
            lands_played_this_turn: 0,
            life: 20,
            mana_pool,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Players {
    inner: Vec<Player>,
}

impl Players {
    pub fn new(num_players: u32) -> Self {
        let inner = (0..num_players)
            .map(|i| Player::new(PlayerId(i), format!("Player {}", i + 1)))
            .collect();

        Self { inner }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn get(&self, id: PlayerId) -> Option<&Player> {
        self.inner.get(id.0 as usize)
    }

    pub fn get_mut(&mut self, id: PlayerId) -> Option<&mut Player> {
        self.inner.get_mut(id.0 as usize)
    }

    pub fn iter(&self) -> impl Iterator<Item = &'_ Player> + '_ {
        self.inner.iter()
    }

    pub fn player_after(&self, id: PlayerId) -> PlayerId {
        let maybe_index = self.inner.iter().position(|turn| turn.id == id);

        let index = match maybe_index {
            Some(index) => index,
            None => panic!("Game::player_after was called with a non-player Entity."),
        };

        let next_index = (index + 1) % self.inner.len();
        self.inner[next_index].id
    }
}

impl<'a> IntoIterator for &'a Players {
    type Item = &'a Player;
    type IntoIter = std::slice::Iter<'a, Player>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'a> IntoIterator for &'a mut Players {
    type Item = &'a mut Player;
    type IntoIter = std::slice::IterMut<'a, Player>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}
