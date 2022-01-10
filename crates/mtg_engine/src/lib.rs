pub mod action;
pub mod card;
pub mod components;
pub mod cost;
pub mod counters;
pub mod game;
pub mod ident;
pub mod keyword_ability;
pub mod mana_pool;
pub mod object_db;
pub mod player;
pub mod pt;
pub mod queries;
pub mod types;
pub mod zone;

pub use hecs;

#[cfg(test)]
mod test {
    use crate::components::UntilEotEffect;
    use crate::game::{util::advance_with_no_actions, Game};
    use crate::pt::{AdjustPtEffect, PtValue};
    use crate::queries::QueryPt;
    use crate::zone::ZoneId;

    #[test]
    fn until_eot_pt_adjust() {
        let mut game = Game::new();
        let grizzly_bears = game.object_db().card_id("Grizzly Bears").unwrap();

        let player1 = game.players().iter().next().unwrap().id;
        let bear = game
            .create_card(grizzly_bears, ZoneId::Battlefield, player1)
            .unwrap();

        let giant_growth = game.world_mut().spawn((
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
            advance_with_no_actions(&mut game);
        }

        assert!(
            !game.world().contains(giant_growth),
            "Giant Growth was not cleaned up at turn end"
        );
        let bear_pt = game.query(QueryPt(bear)).unwrap();
        assert_eq!(bear_pt, PtValue::new(2, 2));
    }

    #[test]
    fn turns_pass() {
        let mut game = Game::new();

        while game.turn_number() < 3 {
            advance_with_no_actions(&mut game);
        }
    }
}
