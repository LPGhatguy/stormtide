mod pt;

use std::collections::BTreeSet;
use std::mem::swap;

use hecs::{Entity, World};

use pt::{AdjustPtEffect, PtCharacteristic, PtValue, SetPtEffect, SwitchPtEffect};

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

/// A component that indicates that this entity is a player.
#[derive(Debug)]
struct Player;

/// A component to indicate that this entity is an effect that should be cleaned
/// up at the end of the turn.
#[derive(Debug)]
struct UntilEotEffect;

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
    pt: PtCharacteristic,
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

        // Layer 7a: characteristic-defining P/T.
        let mut calculated_pt = creature.pt.resolve();

        // Layer 7b: any effects that directly set power/toughness.
        //
        // TODO: Sort by timestamp.
        let mut layer_7b_query = world.query::<(&SetPtEffect,)>();
        for (_entity, (effect,)) in layer_7b_query.iter() {
            if effect.target == self.0 {
                calculated_pt = effect.value;
            }
        }

        // Layer 7c: any effects that adjust power/toughness without setting it.
        //
        // TODO: Sort by timestamp.
        let mut layer_7c_query = world.query::<(&AdjustPtEffect,)>();
        for (_entity, (effect,)) in layer_7c_query.iter() {
            if effect.target == self.0 {
                calculated_pt.power += effect.adjustment.power;
                calculated_pt.toughness += effect.adjustment.toughness;
            }
        }

        // Layer 7d: power/toughness adjustments from counters
        //
        // TODO

        // Layer 7e: power/toughness switching
        let mut layer_7e_query = world.query::<(&SwitchPtEffect,)>();
        for (_entity, (effect,)) in layer_7e_query.iter() {
            if effect.target == self.0 {
                swap(&mut calculated_pt.power, &mut calculated_pt.toughness);
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
            pt: PtCharacteristic::Normal(PtValue {
                power: 2,
                toughness: 2,
            }),
        },
        Permanent { tapped: false },
    ));

    let giant_growth = world.spawn((
        UntilEotEffect,
        AdjustPtEffect {
            target: bear,
            adjustment: PtValue {
                power: 3,
                toughness: 3,
            },
        },
    ));

    let bear_pt = QueryPt(bear).query(&world);
    println!("bear: {:?}", bear_pt);
}
