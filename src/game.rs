//! Defines the high-level structure describing a game of Magic.

use std::collections::HashMap;

use hecs::{Entity, World};

use crate::action::Action;
use crate::components::Player;
use crate::queries::Query;

#[allow(unused)]
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

        let player1 = world.spawn((Player::new(),));
        let player2 = world.spawn((Player::new(),));

        let players = vec![player1, player2];

        let mut zones = maplit::hashmap! {
            ZoneId::Stack => Zone::new(),
            ZoneId::Battlefield => Zone::new(),
            ZoneId::Exile => Zone::new(),
            ZoneId::Command => Zone::new(),
        };

        for &player in &players {
            zones.insert(ZoneId::Library(player), Zone::new());
            zones.insert(ZoneId::Hand(player), Zone::new());
            zones.insert(ZoneId::Graveyard(player), Zone::new());
        }

        Self {
            world,
            turn_order: players,
            active_player: player1,
            priority: Some(player1),
            step: Step::Upkeep,
            zones,
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

    /// 704. State-Based Actions (https://mtg.gamepedia.com/State-based_action)
    fn apply_state_based_actions(&mut self) {
        // 704.5a If a player has 0 or less life, that player loses the game.
        {
            let mut player_query = self.world.query::<(&Player,)>();

            for (entity, (player,)) in player_query.iter() {
                if !player.has_lost && player.life <= 0 {
                    // TODO: Check if player is exempt from this SBA, like via
                    // Phyrexian Unlife.
                    player.has_lost = true;
                }
            }
        }

        // 704.5b If a player attempted to draw a card from a library with no
        //        cards in it since the last time state-based actions were
        //        checked, that player loses the game.
        //
        // TODO

        // 704.5c If a player has ten or more poison counters, that player loses
        //        the game. Ignore this rule in Two-Headed Giant games; see rule
        //        704.6b instead.
        //
        // TODO

        // 704.5d If a token is in a zone other than the battlefield, it ceases
        //        to exist.
        //
        // TODO

        // 704.5e If a copy of a spell is in a zone other than the stack, it
        //        ceases to exist. If a copy of a card is in any zone other than
        //        the stack or the battlefield, it ceases to exist.
        //
        // TODO

        // 704.5f If a creature has toughness 0 or less, it’s put into its
        //        owner’s graveyard. Regeneration can’t replace this event.
        //
        // TODO

        // 704.5g If a creature has toughness greater than 0, it has damage
        //        marked on it, and the total damage marked on it is greater
        //        than or equal to its toughness, that creature has been dealt
        //        lethal damage and is destroyed. Regeneration can replace this
        //        event.
        //
        // TODO

        // 704.5h If a creature has toughness greater than 0, and it’s been
        //        dealt damage by a source with deathtouch since the last time
        //        state-based actions were checked, that creature is destroyed.
        //        Regeneration can replace this event.
        //
        // TODO

        // 704.5i If a planeswalker has loyalty 0, it’s put into its owner’s
        //        graveyard.
        //
        // TODO

        // 704.5j If a player controls two or more legendary permanents with the
        //        same name, that player chooses one of them, and the rest are
        //        put into their owners’ graveyards. This is called the “legend
        //        rule.”
        //
        // TODO

        // 704.5k If two or more permanents have the supertype world, all except
        //        the one that has had the world supertype for the shortest
        //        amount of time are put into their owners’ graveyards. In the
        //        event of a tie for the shortest amount of time, all are put
        //        into their owners’ graveyards. This is called the “world
        //        rule.”
        //
        // TODO

        // 704.5m If an Aura is attached to an illegal object or player, or is
        //        not attached to an object or player, that Aura is put into its
        //        owner’s graveyard.
        //
        // TODO

        // 704.5n If an Equipment or Fortification is attached to an illegal
        //        permanent or to a player, it becomes unattached from that
        //        permanent or player. It remains on the battlefield.
        //
        // TODO

        // 704.5p If a creature is attached to an object or player, it becomes
        //        unattached and remains on the battlefield. Similarly, if a
        //        permanent that’s neither an Aura, an Equipment, nor a
        //        Fortification is attached to an object or player, it becomes
        //        unattached and remains on the battlefield.
        //
        // TODO

        // 704.5q If a permanent has both a +1/+1 counter and a -1/-1 counter on
        //        it, N +1/+1 and N -1/-1 counters are removed from it, where N
        //        is the smaller of the number of +1/+1 and -1/-1 counters on
        //        it.
        //
        // TODO

        // 704.5r If a permanent with an ability that says it can’t have more
        //        than N counters of a certain kind on it has more than N
        //        counters of that kind on it, all but N of those counters are
        //        removed from it.
        //
        // TODO

        // 704.5s If the number of lore counters on a Saga permanent is greater
        //        than or equal to its final chapter number and it isn’t the
        //        source of a chapter ability that has triggered but not yet
        //        left the stack, that Saga’s controller sacrifices it. See rule
        //        714, “Saga Cards.”
        //
        // TODO
    }

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
#[allow(unused)]
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
#[allow(unused)]
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
