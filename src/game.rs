//! Defines the high-level structure describing a game of Magic.

use std::collections::{HashMap, HashSet};
use std::fmt::{self, Debug};

use hecs::{Entity, World};

use crate::action::Action;
use crate::components::{AttachedToEntity, Player};
use crate::queries::Query;

#[allow(unused)]
pub struct Game {
    /// The source of information for all game objects in all zones, as well as
    /// active effects and anything that can be targeted.
    pub world: World,

    /// The next timestamp that will be assigned to an entity.
    next_timestamp: u64,

    /// The turn order and list of player entities in the game.
    pub turn_order: Vec<Entity>,

    /// The current turn. Starts at 0 before the first untap step, then proceeds
    /// at the end of each round of turns.
    pub turn_number: usize,

    /// For this round of priority passing, tracks which players have had a
    /// chance to take an action and have already passed priority.
    ///
    /// When all players have passed priority, the step and/or turn advances.
    players_that_have_passed: HashSet<Entity>,

    /// The Active Player (AP) is the player whose turn it is. All other players
    /// are Non-Active Players (NAP).
    pub active_player: Entity,

    /// The player, if any, that has priority right now. In some steps, like the
    /// untap and cleanup steps, players do not normally receive priority.
    pub priority_player: Option<Entity>,

    /// The current step in the game.
    step: Step,

    /// The current state of the game; what the game dictates must happen next
    /// to proceed.
    state: GameState,

    /// Tracks all zones in the game, used as an index into `world`, which
    /// contains this information as well on each entity.
    zones: HashMap<ZoneId, Zone>,

    // TODO: Triggered abilities that haven't been placed on the stack yet.
    // These abilities will be placed on an order in APNAP order, with each
    // player choosing how to order the individual triggers.
    pending_triggers: (),
}

pub enum GameState {
    /// The game is processing. No players can do anything right now.
    Processing,

    /// A player has priority and can start to take an action. `priority_player`
    /// will be Some.
    Priority,

    /// The game has concluded.
    Complete(GameOutcome),

    /// The game needs a specific kind of input to continue, like choosing a
    /// spell's target, choosing how to pay a cost, etc.
    ///
    /// This can also be used to pause the rules engine while a request is being
    /// made to a hyptothetical authoritative server for more information.
    NeedInput(GameInput),
}

pub enum GameOutcome {
    Win(Entity),
}

pub struct GameInput {
    player: Entity,
    input: GameInputKind,
}

pub enum GameInputKind {
    ChooseAttackers,
    ChooseBlockers,
    ChooseBlockerOrder,

    ChooseTarget {
        options: Box<dyn Query<Output = Vec<Entity>>>,
    },
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
            next_timestamp: 0,
            turn_order: players,
            turn_number: 1,
            players_that_have_passed: HashSet::new(),
            active_player: player1,
            priority_player: Some(player1),
            step: Step::Upkeep,
            state: GameState::Priority,
            zones,
            pending_triggers: (),
        }
    }

    /// Resolve a given query to compute a property of the game state, like a
    /// property of a game object.
    pub fn query<Q: Query>(&self, query_object: Q) -> Q::Output {
        query_object.query(&self.world)
    }

    pub fn possible_actions(&self, player: Entity) -> Vec<Action> {
        let mut actions = vec![Action::Concede];

        if self.priority_player == Some(player) {
            actions.push(Action::PassPriority);
        }

        actions
    }

    pub fn do_action(&mut self, player: Entity, action: Action) {
        log::debug!("Player {:?} attempting action {:?}", player, action);

        match action {
            Action::Concede => {
                todo!("let player {:?} concede", player)
            }
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

    pub fn timestamp(&mut self) -> u64 {
        let timestamp = self.next_timestamp;
        self.next_timestamp += 1;
        timestamp
    }

    /// 704. State-Based Actions (https://mtg.gamepedia.com/State-based_action)
    fn apply_state_based_actions(&mut self) {
        // Clear any effects attached to objects that no longer exist.
        {
            let mut entities_to_despawn = Vec::new();
            let mut query = self.world.query::<(&AttachedToEntity,)>();

            // Should this be applied recursively? For simplicity, it is not,
            // but this means that nested chains of attachments may not be
            // removed correctly.
            for (entity, (attached,)) in query.iter() {
                if !self.world.contains(attached.target) {
                    entities_to_despawn.push(entity);
                }
            }

            drop(query);

            for entity in entities_to_despawn {
                self.world.despawn(entity).unwrap();
            }
        }

        // 704.5a If a player has 0 or less life, that player loses the game.
        {
            let mut player_query = self.world.query::<(&mut Player,)>();

            for (_entity, (player,)) in player_query.iter() {
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
        if self.priority_player != Some(player) {
            log::warn!(
                "Player {:?} tried to pass priority but is not the priority player",
                player
            );
            return;
        }

        self.players_that_have_passed.insert(player);

        // 117.4. If all players pass in succession (that is, if all players
        //        pass without taking any actions in between passing), the spell
        //        or ability on top of the stack resolves or, if the stack is
        //        empty, the phase or step ends.
        let next_player = self.player_after(player);
        log::debug!("Player {:?} passing priority to {:?}", player, next_player);

        if self.players_that_have_passed.contains(&next_player) {
            log::debug!("All players have passed");

            self.priority_player = None;
            self.players_that_have_passed.clear();

            let stack = &self.zones[&ZoneId::Stack];
            if stack.is_empty() {
                self.end_current_step();
            } else {
                self.resolve_one_from_stack();
            }
        } else {
            // TODO: Do we need to apply state based actions here?
            self.priority_player = Some(next_player);
        }
    }

    fn end_current_step(&mut self) {
        log::debug!("Ending current step");

        let stack = &self.zones[&ZoneId::Stack];
        assert!(stack.is_empty());

        if let Some(next_step) = self.next_step() {
            // Advancing to the next step within the same turn.
            self.enter_step(next_step);
        } else {
            // Advacing to the next turn.
            self.end_current_turn();
        }
    }

    fn end_current_turn(&mut self) {
        log::debug!("Ending current turn");

        let stack = &self.zones[&ZoneId::Stack];
        assert!(stack.is_empty());

        let next_player = self.player_after(self.active_player);
        let is_new_turn_cycle = next_player == self.turn_order[0];

        self.active_player = next_player;
        self.enter_step(Step::Untap);

        if is_new_turn_cycle {
            self.turn_number += 1;
        }
    }

    fn enter_step(&mut self, step: Step) {
        log::debug!("Entering step {:?}", step);

        self.step = step;

        match step {
            // 502. Untap Step
            Step::Untap => {
                // 502.1. First, all phased-in permanents with phasing that the
                //        active player controls phase out, and all phased-out
                //        permanents that the active player controlled when they
                //        phased out phase in. This all happens simultaneously.
                //        This turn-based action doesn’t use the stack. See rule
                //        702.25, “Phasing.”
                //
                // TODO

                // 502.2. Second, the active player determines which permanents
                //        they control will untap. Then they untap them all
                //        simultaneously. This turn-based action doesn’t use the
                //        stack. Normally, all of a player’s permanents untap,
                //        but effects can keep one or more of a player’s
                //        permanents from untapping.
                //
                // TODO

                // 502.3. No player receives priority during the untap step, so
                //        no spells can be cast or resolve and no abilities can
                //        be activated or resolve. Any ability that triggers
                //        during this step will be held until the next time a
                //        player would receive priority, which is usually during
                //        the upkeep step. (See rule 503, “Upkeep Step.”)
                self.enter_step(Step::Upkeep);
            }

            // 503. Upkeep Step
            Step::Upkeep => {
                // 503.1. The upkeep step has no turn-based actions. Once it
                //        begins, the active player gets priority. (See rule
                //        117, “Timing and Priority.”)
                self.priority_player = Some(self.active_player);

                // 503.1a Any abilities that triggered during the untap step and
                //        any abilities that triggered at the beginning of the
                //        upkeep are put onto the stack before the active player
                //        gets priority; the order in which they triggered
                //        doesn’t matter. (See rule 603, “Handling Triggered
                //        Abilities.”)
                //
                // TODO
            }

            // 504. Draw Step
            Step::Draw => {
                // 504.1. First, the active player draws a card. This turn-based
                //        action doesn’t use the stack.
                //
                // TODO

                // 504.2. Second, the active player gets priority. (See rule
                //        117, “Timing and Priority.”)
                self.priority_player = Some(self.active_player);
            }

            // 505. Main Phase
            Step::Main1 | Step::Main2 => {
                // 505.4. Second, if the active player controls one or more Saga
                //        enchantments and it’s the active player’s precombat
                //        main phase, the active player puts a lore counter on
                //        each Saga they control. (See rule 714, “Saga Cards.”)
                //        This turn-based action doesn’t use the stack.
                //
                // TODO

                // 505.5. Third, the active player gets priority. (See rule 117,
                //        “Timing and Priority.”)
                self.priority_player = Some(self.active_player);
            }

            Step::BeginCombat
            | Step::DeclareAttackers
            | Step::DeclareBlockers
            | Step::CombatDamage
            | Step::EndOfCombat => {
                // TODO: just skipping combat completely for now
                self.end_current_step();
            }

            // 513. End Step
            Step::End => {
                // 513.1. The end step has no turn-based actions. Once it
                //        begins, the active player gets priority. (See rule
                //        117, “Timing and Priority.”)
                self.priority_player = Some(self.active_player);
            }

            // 514. Cleanup Step
            Step::Cleanup => {
                // 514.1. First, if the active player’s hand contains more cards
                //        than their maximum hand size (normally seven), they
                //        discard enough cards to reduce their hand size to that
                //        number. This turn-based action doesn’t use the stack.
                //
                // TODO

                // 514.2. Second, the following actions happen simultaneously:
                //        all damage marked on permanents (including phased-out
                //        permanents) is removed and all “until end of turn” and
                //        “this turn” effects end. This turn-based action
                //        doesn’t use the stack.
                //
                // TODO

                // 514.3. Normally, no player receives priority during the
                //        cleanup step, so no spells can be cast and no
                //        abilities can be activated. However, this rule is
                //        subject to the following exception:
                // 514.3a At this point, the game checks to see if any
                //        state-based actions would be performed and/or any
                //        triggered abilities are waiting to be put onto the
                //        stack (including those that trigger “at the beginning
                //        of the next cleanup step”). If so, those state-based
                //        actions are performed, then those triggered abilities
                //        are put on the stack, then the active player gets
                //        priority. Players may cast spells and activate
                //        abilities. Once the stack is empty and all players
                //        pass in succession, another cleanup step begins.
                //
                // TODO

                self.end_current_step();
            }
        }
    }

    fn resolve_one_from_stack(&mut self) {
        todo!("resolve one entry from stack");
    }

    /// Returns the next player, in turn order. This is used for priority
    /// passing, turn order, and various effects.
    fn player_after(&self, player: Entity) -> Entity {
        let maybe_index = self.turn_order.iter().position(|&turn| turn == player);

        let index = match maybe_index {
            Some(index) => index,
            None => panic!("Game::player_after was called with a non-player Entity."),
        };

        let next_index = (index + 1) % self.turn_order.len();
        self.turn_order[next_index]
    }

    /// Returns the next step if there are steps to take still in this turn.
    fn next_step(&self) -> Option<Step> {
        match self.step {
            Step::Untap => Some(Step::Upkeep),
            Step::Upkeep => Some(Step::Draw),
            Step::Draw => Some(Step::Main1),
            Step::Main1 => Some(Step::BeginCombat),
            Step::BeginCombat => Some(Step::DeclareAttackers),
            Step::DeclareAttackers => Some(Step::DeclareBlockers),
            Step::DeclareBlockers => Some(Step::CombatDamage),
            Step::CombatDamage => Some(Step::EndOfCombat),
            Step::EndOfCombat => Some(Step::Main2),
            Step::Main2 => Some(Step::End),
            Step::End => Some(Step::Cleanup),
            Step::Cleanup => None,
        }
    }
}

impl Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Turn: #{}   AP: {:?}   PP: {:?}   Step: {:?}",
            self.turn_number, self.active_player, self.priority_player, self.step
        )
    }
}

/// 500.1. A turn consists of five phases, in this order: beginning, precombat
///        main, combat, postcombat main, and ending. Each of these phases takes
///        place every turn, even if nothing happens during the phase. The
///        beginning, combat, and ending phases are further broken down into
///        steps, which proceed in order.
#[allow(unused)]
#[derive(Debug, Clone, Copy)]
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

    fn is_empty(&self) -> bool {
        self.members.is_empty()
    }
}
