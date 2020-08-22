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

#[derive(Debug, Clone, Copy)]
struct Pt {
    power: i64,
    toughness: i64,
}

#[derive(Debug)]
struct Player;

#[derive(Debug)]
struct UntilEotEffect;

#[derive(Debug)]
struct PtEffect {
    target: Entity,
    adjustment: Pt,
}

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
    type Output = Option<Pt>;

    fn query(&self, world: &World) -> Self::Output {
        let mut query = world.query_one::<(&Permanent, &Creature)>(self.0).ok()?;
        let (_permament, creature) = query.get()?;

        let mut pt = creature.pt;

        // TODO: Loop through all PtEffect where target == self.0 and apply adjustments.

        Some(creature.pt)
    }
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
            pt: Pt {
                power: 2,
                toughness: 2,
            },
        },
        Permanent { tapped: false },
    ));

    let bear_pt = QueryPt(bear).query(&world);
    println!("bear: {:?}", bear_pt);
}
