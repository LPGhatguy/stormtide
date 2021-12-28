pub mod action;
pub mod card;
pub mod components;
pub mod cost;
pub mod counters;
pub mod game;
pub mod ident;
pub mod keyword_ability;
pub mod object_db;
pub mod pt;
pub mod queries;
pub mod types;

pub use hecs;

#[cfg(test)]
mod test {
    use crate::action::Action;
    use crate::components::{Creature, Object, Permanent, UntilEotEffect};
    use crate::game::{Game, ZoneId};
    use crate::ident::Ident;
    use crate::pt::{AdjustPtEffect, PtCharacteristic, PtValue};
    use crate::queries::QueryPt;

    #[test]
    fn until_eot_pt_adjust() {
        let mut game = Game::new();

        let player1 = game.players()[0];

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

        while game.turn_number() < 2 {
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

        while game.turn_number() < 3 {
            game.do_action(game.priority_player().unwrap(), Action::PassPriority);
        }
    }
}
