use mtg_engine::action::PlayerAction;
use mtg_engine::components::UntilEotEffect;
use mtg_engine::game::Game;
use mtg_engine::pt::{AdjustPtEffect, PtValue};
use mtg_engine::queries::QueryPt;
use mtg_engine::zone::ZoneId;

fn main() {
    env_logger::init();

    let mut game = Game::new();

    let forest = game.object_db().card_id("Forest").unwrap();
    let bear = game.object_db().card_id("Grizzly Bears").unwrap();

    let player1 = game.players().iter().next().unwrap().id;

    let _forest1 = game
        .create_card(forest, ZoneId::Battlefield, player1)
        .unwrap();
    let _forest2 = game
        .create_card(forest, ZoneId::Battlefield, player1)
        .unwrap();
    let bear = game
        .create_card(bear, ZoneId::Battlefield, player1)
        .unwrap();

    let _giant_growth = game.world_mut().spawn((
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
        game.do_action(game.priority_player().unwrap(), PlayerAction::PassPriority);

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
