mod components;
mod game;
mod pt;
mod queries;

use components::{Creature, Land, Object, Permanent, UntilEotEffect};
use game::Game;
use pt::{AdjustPtEffect, PtCharacteristic, PtValue, SetPtEffect, SwitchPtEffect};
use queries::QueryPt;

type Subtype = String;

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

fn main() {
    let mut game = Game::new();

    let player1 = game.base_player_turn_order[0];
    let player2 = game.base_player_turn_order[1];

    let forest1 = game.world.spawn((
        Object {
            owner: player1,
            controller: player1,
        },
        Land,
        Permanent { tapped: false },
    ));
    let forest2 = game.world.spawn((
        Object {
            owner: player1,
            controller: player1,
        },
        Land,
        Permanent { tapped: false },
    ));
    let bear = game.world.spawn((
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

    let giant_growth = game.world.spawn((
        UntilEotEffect,
        AdjustPtEffect {
            target: bear,
            adjustment: PtValue {
                power: 3,
                toughness: 3,
            },
        },
    ));

    let bear_pt = game.query(QueryPt(bear));
    println!("bear: {:?}", bear_pt);
}
