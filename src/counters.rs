use crate::ident::Ident;
use crate::keyword_ability::KeywordAbility;
use crate::pt::PtValue;

/// 122. Counters
///
/// 122.1. A counter is a marker placed on an object or player that modifies its
///        characteristics and/or interacts with a rule, ability, or effect.
///        Counters are not objects and have no characteristics. Notably, a
///        counter is not a token, and a token is not a counter. Counters with
///        the same name or description are interchangeable.
#[derive(Debug)]
pub enum Counter {
    /// 122.1a A +X/+Y counter on a creature or on a creature card in a zone
    ///        other than the battlefield, where X and Y are numbers, adds X to
    ///        that object’s power and Y to that object’s toughness. Similarly,
    ///        -X/-Y counters subtract from power and toughness. See rule
    ///        613.4c.
    Pt(PtValue),

    /// 122.1b A keyword counter on a permanent or on a card in a zone other
    ///          than the battlefield causes that object to gain that keyword.
    ///          The keywords that a keyword counter can be are flying, first
    ///          strike, double strike, deathtouch, haste, hexproof,
    ///          indestructible, lifelink, menace, reach, trample, and
    ///          vigilance, as well as any variants of those keywords. See rule
    ///          613.1f.
    KeywordAbility(KeywordAbility),

    /// 122.1c The number of loyalty counters on a planeswalker on the
    ///        battlefield indicates how much loyalty it has. A planeswalker
    ///        with 0 loyalty is put into its owner’s graveyard as a state-based
    ///        action. See rule 704.
    Loyalty(i64),

    /// 122.1d If a player has ten or more poison counters, that player loses
    ///        the game as a state-based action. See rule 704. A player is
    ///        “poisoned” if they have one or more poison counters. (See rule
    ///        810 for additional rules for Two-Headed Giant games.)
    Poison(i64),

    /// Other counters named by cards, like "filibuster counters" from Azor's
    /// Elocutors.
    Other(Ident),
}
