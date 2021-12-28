use mtg_engine::action::Action;
use mtg_engine::components::{Creature, Land, Object, Permanent, UntilEotEffect};
use mtg_engine::game::{Game, ZoneId};
use mtg_engine::ident::Ident;
use mtg_engine::pt::{AdjustPtEffect, PtCharacteristic, PtValue};
use mtg_engine::queries::QueryPt;

fn main() {
    env_logger::init();

    let mut game = Game::new();

    let player1 = game.players()[0];
    let _player2 = game.players()[1];

    let _forest1 = game.world.spawn((
        Object {
            name: Ident::new("Forest"),
            zone: ZoneId::Battlefield,
            owner: player1,
            controller: Some(player1),
        },
        Land,
        Permanent { tapped: false },
    ));
    let _forest2 = game.world.spawn((
        Object {
            name: Ident::new("Forest"),
            zone: ZoneId::Battlefield,
            owner: player1,
            controller: Some(player1),
        },
        Land,
        Permanent { tapped: false },
    ));
    let bear = game.world.spawn((
        Object {
            name: Ident::new("Grizzly Bears"),
            zone: ZoneId::Battlefield,
            owner: player1,
            controller: Some(player1),
        },
        Creature {
            pt: PtCharacteristic::Normal(PtValue {
                power: 2,
                toughness: 2,
            }),
        },
        Permanent { tapped: false },
    ));

    let _giant_growth = game.world.spawn((
        UntilEotEffect,
        AdjustPtEffect {
            target: bear,
            adjustment: PtValue {
                power: 3,
                toughness: 3,
            },
        },
    ));

    let bear_pt = game.query(QueryPt(bear)).unwrap();
    println!("Bear has P/T: {}", bear_pt);

    while game.turn_number() < 3 {
        println!("{:?}", game);
        game.do_action(game.priority_player().unwrap(), Action::PassPriority);

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}