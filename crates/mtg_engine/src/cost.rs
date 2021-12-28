//! 118.1. A cost is an action or payment necessary to take another action or to
//!        stop another action from taking place. To pay a cost, a player
//!        carries out the instructions specified by the spell, ability, or
//!        effect that contains that cost.
//!
//! There are many kinds of cost for various spells and abilities in the game.

use serde::{Deserialize, Serialize};

/// An action or payment necessary to take another action or to stop another
/// action from taking place. See rule 118, "Costs."
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Cost {
    pub items: Vec<CostItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CostItem {
    Tap,

    /// 107.4a There are five primary colored mana symbols: {W} is white, {U}
    ///        blue, {B} black, {R} red, and {G} green. These symbols are used
    ///        to represent colored mana, and also to represent colored mana in
    ///        costs. Colored mana in costs can be paid only with the
    ///        appropriate color of mana. See rule 202, “Mana Cost and Color.”
    ColoredMana(ManaColor),

    /// 107.4b Numerical symbols (such as {1}) and variable symbols (such as
    ///        {X}) represent generic mana in costs. Generic mana in costs can
    ///        be paid with any type of mana. For more information about {X},
    ///        see rule 107.3.
    GenericMana,
    XGenericMana,

    /// 107.4c The colorless mana symbol {C} is used to represent one colorless
    ///        mana, and also to represent a cost that can be paid only with one
    ///        colorless mana.
    ColorlessMana,

    /// 107.4e Hybrid mana symbols are also colored mana symbols. Each one
    ///        represents a cost that can be paid in one of two ways, as
    ///        represented by the two halves of the symbol. A hybrid symbol such
    ///        as {W/U} can be paid with either white or blue mana, and a
    ///        monocolored hybrid symbol such as {2/B} can be paid with either
    ///        one black mana or two mana of any type. A hybrid mana symbol is
    ///        all of its component colors.
    ///
    ///        Example: {G/W}{G/W} can be paid by spending {G}{G}, {G}{W}, or
    ///        {W}{W}.
    HybridMana(ManaColor, ManaColor),
    MonocoloredHybridMana(ManaColor),

    /// 107.4f Phyrexian mana symbols are colored mana symbols: {W/P} is white,
    ///        {U/P} is blue, {B/P} is black, {R/P} is red, and {G/P} is green.
    ///        A Phyrexian mana symbol represents a cost that can be paid either
    ///        with one mana of its color or by paying 2 life.
    ///
    ///        Example: {W/P}{W/P} can be paid by spending {W}{W}, by spending
    ///        {W} and paying 2 life, or by paying 4 life.
    PhyrexianMana(ManaColor),

    /// 107.4h The snow mana symbol {S} represents one mana in a cost. This mana
    ///        can be paid with one mana of any type produced by a snow
    ///        permanent (see rule 205.4g). Effects that reduce the amount of
    ///        generic mana you pay don’t affect {S} costs. Snow is neither a
    ///        color nor a type of mana.
    SnowMana,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ManaColor {
    White,
    Blue,
    Black,
    Red,
    Green,
}
