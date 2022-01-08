use crate::action::{PlayerAction, PlayerActionCategory};

use super::{Game, GameState};

/// Advance the game with no players taking any actions.
pub fn advance_with_no_actions(game: &mut Game) {
    let state = game.state.clone();
    match state {
        GameState::Player { player, action } => match action {
            PlayerActionCategory::Priority => game.do_action(player, PlayerAction::PassPriority),
            PlayerActionCategory::ChooseAttackers => {
                game.do_action(player, PlayerAction::ChooseAttackers { attackers: vec![] })
            }
            PlayerActionCategory::ChooseBlockers => {
                game.do_action(player, PlayerAction::ChooseBlockers { blockers: vec![] })
            }
            _ => {
                panic!("cannot advance without doing nothing: {:?}", game)
            }
        },
        GameState::Complete(_) => {}
    }
}
