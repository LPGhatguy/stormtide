//! Defines the high-level structure describing a game of Magic.

use std::collections::HashMap;

use hecs::{Entity, World};

use crate::action::Action;
use crate::components::Player;
use crate::queries::Query;

pub struct Game {
    pub world: World,
    pub turn_order: Vec<Entity>,
    active_player: Entity,
    priority: Option<Entity>,
    step: Step,
    zones: HashMap<ZoneId, Zone>,
}

impl Game {
    pub fn new() -> Self {
        let mut world = World::new();

        let player1 = world.spawn((Player,));
        let player2 = world.spawn((Player,));

        Self {
            world,
            turn_order: vec![player1, player2],
            active_player: player1,
            priority: Some(player1),
            step: Step::Upkeep,
            zones: HashMap::new(),
        }
    }

    /// Resolve a given query to compute a property of the game state, like a
    /// property of a game object.
    pub fn query<Q: Query>(&self, query_object: Q) -> Q::Output {
        query_object.query(&self.world)
    }

    pub fn possible_actions(&self, player: Entity) -> Vec<Action> {
        let mut actions = vec![Action::Concede];

        if self.priority == Some(player) {
            actions.push(Action::PassPriority);
        }

        actions
    }

    pub fn do_action(&mut self, player: Entity, action: Action) {
        match action {
            Action::Concede => unimplemented!("complete game"),
            Action::PassPriority => self.pass_priority(player),
            Action::CastSpell { spell } => {
                unimplemented!("player {:?} casting spell {:?}", player, spell)
            }
            Action::ActivateAbility { object, ability } => unimplemented!(
                "player {:?} activating ability #{} on object {:?}",
                player,
                ability,
                object
            ),
            Action::PlayLand { card } => {
                unimplemented!("player {:?} playing land {:?}", player, card)
            }
        }
    }

    fn apply_state_based_actions(&mut self) {}

    fn pass_priority(&mut self, player: Entity) {
        if self.priority != Some(player) {
            return;
        }
    }
}

/// 500.1. A turn consists of five phases, in this order: beginning, precombat
///        main, combat, postcombat main, and ending. Each of these phases takes
///        place every turn, even if nothing happens during the phase. The
///        beginning, combat, and ending phases are further broken down into
///        steps, which proceed in order.
#[derive(Debug)]
pub enum Step {
    // 501.1. The beginning phase consists of three steps, in this order: untap,
    //        upkeep, and draw.
    Untap,
    Upkeep,
    Draw,

    // 505.1. There are two main phases in a turn. In each turn, the first main
    //        phase (also known as the precombat main phase) and the second main
    //        phase (also known as the postcombat main phase) are separated by
    //        the combat phase (see rule 506, “Combat Phase”). The precombat and
    //        postcombat main phases are individually and collectively known as
    //        the main phase.
    Main1,
    Main2,

    // 506.1. The combat phase has five steps, which proceed in order:
    //        beginning of combat, declare attackers, declare blockers, combat
    //        damage, and end of combat. The declare blockers and combat damage
    //        steps are skipped if no creatures are declared as attackers or
    //        put onto the battlefield attacking (see rule 508.8). There are
    //        two combat damage steps if any attacking or blocking creature has
    //        first strike (see rule 702.7) or double strike (see rule 702.4).
    BeginCombat,
    DeclareAttackers,
    DeclareBlockers,
    CombatDamage,
    EndOfCombat,

    // 512.1. The ending phase consists of two steps: end and cleanup.
    End,
    Cleanup,
}

/// 400.1. A zone is a place where objects can be during a game. There are
///        normally seven zones: library, hand, battlefield, graveyard, stack,
///        exile, and command. Some older cards also use the ante zone. Each
///        player has their own library, hand, and graveyard. The other zones
///        are shared by all players.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ZoneId {
    Library(Entity),
    Hand(Entity),
    Graveyard(Entity),
    Stack,
    Battlefield,
    Exile,
    Command,
}

#[derive(Debug)]
struct Zone {
    members: Vec<Entity>,
}

impl Zone {
    fn new() -> Self {
        Self {
            members: Vec::new(),
        }
    }
}
