use std::collections::BTreeSet;

use hecs::{Entity, World};

type Subtype = String;

#[derive(Debug)]
enum Zone {
    Hand,
    Battlefield,
    Graveyard,
    Exile,
}

/// 205.2a The card types are artifact, conspiracy, creature, enchantment,
///        instant, land, phenomenon, plane, planeswalker, scheme, sorcery,
///        tribal, and vanguard. See section 3, “Card Types.”
#[derive(Debug)]
enum CardType {
    Artifact,
    // Conspiracy,
    Creature,
    Enchantment,
    Instant,
    // Phenomenon,
    // Plane,
    Planeswalker,
    // Scheme,
    Sorcery,
    Tribal,
    // Vanguard,
}

/// Represent base power and toughness that can be present on a creature. Some
/// creatures have special rules that determine their base power and toughness,
/// which can be further affected by other effects.
#[derive(Debug, Clone, Copy)]
enum Pt {
    Normal(PtValue),
}

impl Pt {
    /// This method will need access to the game state to do things like:
    /// - count the number of card types in all graveyards (Tarmogoyf)
    /// - count the number of cards in its owner's hand (Maro)
    fn resolve(&self) -> PtValue {
        match self {
            Pt::Normal(value) => *value,
        }
    }
}

/// Represents a change that can be applied to a creature's power and toughness.
#[derive(Debug, Clone, Copy)]
enum PtAdjustment {
    Fixed(PtValue),
}

impl PtAdjustment {
    fn adjust(&self, value: PtValue) -> PtValue {
        match self {
            PtAdjustment::Fixed(adjustment) => PtValue {
                power: value.power + adjustment.power,
                toughness: value.toughness + adjustment.toughness,
            },
        }
    }
}

/// Container for power and toughness, helping simplify calculations.
#[derive(Debug, Clone, Copy)]
struct PtValue {
    power: i64,
    toughness: i64,
}

/// A component that indicates that this entity is a player.
#[derive(Debug)]
struct Player;

/// A component to indicate that this entity is an effect that should be cleaned
/// up at the end of the turn.
#[derive(Debug)]
struct UntilEotEffect;

#[derive(Debug)]
struct PtEffect {
    target: Entity,
    adjustment: PtAdjustment,
}

/// 109.1. An object is an ability on the stack, a card, a copy of a card, a
///        token, a spell, a permanent, or an emblem.
#[derive(Debug)]
struct Object {
    owner: Entity,
    controller: Entity,
}

#[derive(Debug)]
struct Spell {
    targets: BTreeSet<Entity>,
}

#[derive(Debug)]
struct Permanent {
    tapped: bool,
}

#[derive(Debug)]
struct Creature {
    pt: Pt,
}

#[derive(Debug)]
struct Land;

trait Query {
    type Output;

    fn query(&self, world: &World) -> Self::Output;
}

struct QueryPt(Entity);

impl Query for QueryPt {
    type Output = Option<PtValue>;

    fn query(&self, world: &World) -> Self::Output {
        let mut query = world.query_one::<(&Permanent, &Creature)>(self.0).ok()?;
        let (_permament, creature) = query.get()?;

        let mut calculated_pt = creature.pt.resolve();

        let mut effect_query = world.query::<(&PtEffect,)>();
        for (_entity, (effect,)) in effect_query.iter() {
            if effect.target == self.0 {
                calculated_pt = effect.adjustment.adjust(calculated_pt);
            }
        }

        Some(calculated_pt)
    }
}

struct Game {
    world: World,
    base_player_turn_order: Vec<Entity>,
}

fn main() {
    let mut world = World::new();

    let player1 = world.spawn((Player,));
    let player2 = world.spawn((Player,));

    let forest1 = world.spawn((
        Object {
            owner: player1,
            controller: player1,
        },
        Land,
        Permanent { tapped: false },
    ));
    let forest2 = world.spawn((
        Object {
            owner: player1,
            controller: player1,
        },
        Land,
        Permanent { tapped: false },
    ));
    let bear = world.spawn((
        Object {
            owner: player1,
            controller: player1,
        },
        Creature {
            pt: Pt::Normal(PtValue {
                power: 2,
                toughness: 2,
            }),
        },
        Permanent { tapped: false },
    ));

    let giant_growth = world.spawn((
        UntilEotEffect,
        PtEffect {
            target: bear,
            adjustment: PtAdjustment::Fixed(PtValue {
                power: 3,
                toughness: 3,
            }),
        },
    ));

    let bear_pt = QueryPt(bear).query(&world);
    println!("bear: {:?}", bear_pt);
}
