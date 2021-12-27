mod action;
mod card;
mod components;
mod cost;
mod counters;
mod game;
mod ident;
mod keyword_ability;
mod pt;
mod queries;
mod types;

use action::Action;
use components::{Creature, Land, Object, Permanent, UntilEotEffect};
use game::Game;
use pt::{AdjustPtEffect, PtCharacteristic, PtValue};
use queries::QueryPt;

fn main() {
    env_logger::init();

    let mut game = Game::new();

    let player1 = game.turn_order[0];
    let player2 = game.turn_order[1];

    let _forest1 = game.world.spawn((
        Object {
            owner: player1,
            controller: player1,
        },
        Land,
        Permanent { tapped: false },
    ));
    let _forest2 = game.world.spawn((
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

    while game.turn_number < 3 {
        println!("{:?}", game);
        game.do_action(game.priority_player().unwrap(), Action::PassPriority);

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

#[cfg(test)]
mod test {
    use crate::action::Action;
    use crate::components::{Creature, Land, Object, Permanent, UntilEotEffect};
    use crate::game::Game;
    use crate::pt::{AdjustPtEffect, PtCharacteristic, PtValue};
    use crate::queries::QueryPt;

    #[test]
    fn until_eot_pt_adjust() {
        let mut game = Game::new();

        let player1 = game.turn_order[0];

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

        let bear_pt = game.query(QueryPt(bear)).unwrap();
        assert_eq!(bear_pt, PtValue::new(5, 5));

        while game.turn_number < 2 {
            game.do_action(game.priority_player().unwrap(), Action::PassPriority);
        }

        assert!(
            game.world.entity(giant_growth).is_err(),
            "Giant Growth was not cleaned up at turn end"
        );
        let bear_pt = game.query(QueryPt(bear)).unwrap();
        assert_eq!(bear_pt, PtValue::new(2, 2));
    }

    #[test]
    fn turns_pass() {
        let mut game = Game::new();

        while game.turn_number < 3 {
            game.do_action(game.priority_player().unwrap(), Action::PassPriority);
        }
    }
}
