//! 118.1. A cost is an action or payment necessary to take another action or to
//!        stop another action from taking place. To pay a cost, a player
//!        carries out the instructions specified by the spell, ability, or
//!        effect that contains that cost.
//!
//! There are many kinds of cost for various spells and abilities in the game.
//!
//!

pub struct Cost {
    pub mana: Vec<ManaCost>,
    pub tap: bool,
}

/// TODO
pub struct ManaCost;
