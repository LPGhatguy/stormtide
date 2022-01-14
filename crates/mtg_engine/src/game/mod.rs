//! Defines the high-level structure describing a game of Magic.

mod casting;
mod combat;
mod state_based_actions;
pub mod util;

use std::collections::{HashMap, HashSet};
use std::fmt::{self, Debug};

use hecs::{Entity, EntityBuilder, World};
use serde::{Deserialize, Serialize};

use crate::player::{PlayerId, Players};
use crate::{
    action::{PlayerAction, PlayerActionCategory},
    components::{Card, Damage, Object, Permanent, UntilEotEffect},
    object_db::{CardId, ObjectDb},
    queries::Query,
    types::CardType,
    zone::{Zone, ZoneId},
};

pub struct Game {
    /// A database containing objects that can be instantiated into the game.
    object_db: ObjectDb,

    /// The source of information for all game objects in all zones, as well as
    /// active effects and anything that can be targeted.
    world: World,

    /// The next timestamp that will be assigned to an entity.
    next_timestamp: u64,

    players: Players,

    /// The current turn. Starts at 0 before the first untap step, then proceeds
    /// at the end of each round of turns.
    turn_number: u64,

    /// For this round of priority passing, tracks which players have had a
    /// chance to take an action and have already passed priority.
    ///
    /// When all players have passed priority, the step and/or turn advances.
    players_that_have_passed: HashSet<PlayerId>,

    /// The Active Player (AP) is the player whose turn it is. All other players
    /// are Non-Active Players (NAP).
    active_player: PlayerId,

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
    _pending_triggers: (),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GameState {
    /// The game is waiting on a player to do something. This can vary from a
    /// player having priority, to choosing a spell's target or how to pay a
    /// cost.
    Player {
        player: PlayerId,
        action: PlayerActionCategory,
    },

    /// The game has concluded.
    Complete(GameOutcome),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GameOutcome {
    Win { winner: PlayerId },
}

impl Game {
    pub fn new() -> Self {
        let object_db = ObjectDb::load();
        let world = World::new();

        let players = Players::new(2);

        let mut zones = maplit::hashmap! {
            ZoneId::Stack => Zone::new(),
            ZoneId::Battlefield => Zone::new(),
            ZoneId::Exile => Zone::new(),
            ZoneId::Command => Zone::new(),
        };

        for player in &players {
            zones.insert(ZoneId::Library(player.id), Zone::new());
            zones.insert(ZoneId::Hand(player.id), Zone::new());
            zones.insert(ZoneId::Graveyard(player.id), Zone::new());
        }

        let player1_id = players.iter().next().unwrap().id;

        Self {
            object_db,
            world,
            next_timestamp: 0,
            players,
            turn_number: 1,
            players_that_have_passed: HashSet::new(),
            active_player: player1_id,
            step: Step::Upkeep,
            state: GameState::Player {
                player: player1_id,
                action: PlayerActionCategory::Priority,
            },
            zones,
            _pending_triggers: (),
        }
    }

    /// Temporary method to update our zones index from world information.
    pub fn rebuild_zone_index(&mut self) {
        for zone in self.zones.values_mut() {
            zone.clear();
        }

        for (entity, (object,)) in self.world.query_mut::<(&Object,)>() {
            if let Some(zone) = self.zones.get_mut(&object.zone) {
                zone.add(entity);
            }
        }
    }

    pub fn do_action(&mut self, player: PlayerId, action: PlayerAction) {
        log::debug!("Player {:?} attempting action {:?}", player, action);

        match action {
            PlayerAction::Concede => self.player_loses(player),
            PlayerAction::PassPriority => self.pass_priority(player),

            PlayerAction::ChooseAttackers { attackers } => {
                combat::choose_attackers(self, player, &attackers)
            }

            PlayerAction::ChooseBlockers { blockers } => {
                combat::choose_blockers(self, player, &blockers)
            }

            PlayerAction::PlayLand { card } => self.play_land(player, card),
            PlayerAction::StartCastingSpell { spell } => {
                casting::start_casting_spell(self, player, spell)
            }
            PlayerAction::FinishCastingSpell { spell } => {
                casting::finish_casting_spell(self, player, spell)
            }
            PlayerAction::CancelCastingSpell { spell } => {
                casting::cancel_casting_spell(self, player, spell)
            }
            PlayerAction::PayIncompleteSpellMana { spell, mana } => {
                casting::pay_spell_mana(self, player, spell, mana)
            }
        }
    }

    pub fn create_card(&mut self, id: CardId, zone_id: ZoneId, owner: PlayerId) -> Option<Entity> {
        let zone = self.zones.get_mut(&zone_id)?;
        let descriptor = self.object_db.card(id)?;

        // 109.4. Only objects on the stack or on the battlefield have a
        //        controller. Objects that are neither on the stack nor on the
        //        battlefield aren’t controlled by any player. See rule 108.4.
        let controller = if zone_id == ZoneId::Battlefield || zone_id == ZoneId::Stack {
            Some(owner)
        } else {
            None
        };

        let mut builder = EntityBuilder::new();
        builder.add(Object {
            name: descriptor.name.clone(),
            types: descriptor.types.clone(),
            supertypes: descriptor.supertypes.clone(),
            subtypes: descriptor.subtypes.clone(),
            mana_cost: descriptor.mana_cost.clone(),
            pt: descriptor.pt,
            zone: zone_id,
            owner,
            controller,
        });
        builder.add(Card { id });

        // 110.1. A permanent is a card or token on the battlefield. A permanent
        //        remains on the battlefield indefinitely. A card or token
        //        becomes a permanent as it enters the battlefield and it stops
        //        being a permanent as it’s moved to another zone by an effect
        //        or rule.
        if zone_id == ZoneId::Battlefield {
            builder.add(Permanent { tapped: false });
        }

        let entity = self.world.spawn(builder.build());
        zone.add(entity);

        Some(entity)
    }

    pub fn move_object_to_zone(&mut self, object_id: Entity, zone_id: ZoneId) -> Option<()> {
        if !self.zones.contains_key(&zone_id) {
            log::warn!(
                "Cannot move object {:?} to zone {:?}: the zone does not exist",
                object_id,
                zone_id,
            );
            return None;
        }

        let mut object = self.world.get_mut::<Object>(object_id).ok()?;
        let old_zone_id = object.zone;
        if zone_id == old_zone_id {
            log::warn!(
                "Cannot move object {:?} to zone {:?}: it is already in that zone",
                object_id,
                zone_id,
            );
            return Some(());
        }

        let old_zone = self.zones.get_mut(&old_zone_id)?;
        old_zone.remove(object_id);

        // Panic safety: checked when calling contains_key above
        let new_zone = self.zones.get_mut(&zone_id).unwrap();
        object.zone = zone_id;
        new_zone.add(object_id);

        // 110.2. A permanent’s owner is the same as the owner of the card that
        //        represents it (unless it’s a token; see rule 111.2). A
        //        permanent’s controller is, by default, the player under whose
        //        control it entered the battlefield. Every permanent has a
        //        controller.
        //
        // 110.2a If an effect instructs a player to put an object onto the
        //        battlefield, that object enters the battlefield under that
        //        player’s control unless the effect states otherwise.
        object.controller = Some(object.owner);

        drop(object);

        // 110.1. A permanent is a card or token on the battlefield. A permanent
        //        remains on the battlefield indefinitely. A card or token
        //        becomes a permanent as it enters the battlefield and it stops
        //        being a permanent as it’s moved to another zone by an effect
        //        or rule.
        if zone_id == ZoneId::Battlefield {
            self.world
                .insert_one(object_id, Permanent { tapped: false })
                .unwrap();
        } else if old_zone_id == ZoneId::Battlefield {
            let _ = self.world.remove_one::<Permanent>(object_id);
        }

        Some(())
    }

    pub fn object_db(&self) -> &ObjectDb {
        &self.object_db
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    /// This function will go away at some point.
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    /// Resolve a given query to compute a property of the game state, like a
    /// property of a game object.
    pub fn query<Q: Query>(&self, query_object: Q) -> Q::Output {
        query_object.query(self)
    }

    pub fn active_player(&self) -> PlayerId {
        self.active_player
    }

    pub fn turn_number(&self) -> u64 {
        self.turn_number
    }

    pub fn zone(&self, id: ZoneId) -> Option<&Zone> {
        self.zones.get(&id)
    }

    pub fn zone_mut(&mut self, id: ZoneId) -> Option<&mut Zone> {
        self.zones.get_mut(&id)
    }

    /// Returns all players in turn order.
    pub fn players(&self) -> &Players {
        &self.players
    }

    pub fn step(&self) -> Step {
        self.step
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }

    /// Check state-based actions and then give a player priority as long as the
    /// game hasn't ended as a result.
    pub fn give_priority(&mut self, player: PlayerId) {
        self.apply_state_based_actions();

        if matches!(self.state, GameState::Complete(_)) {
            return;
        }

        self.state = GameState::Player {
            player,
            action: PlayerActionCategory::Priority,
        };
    }

    /// Give a player priority and reset the state tracking who has passed. This
    /// should be used to give players priority after a player takes an action
    /// so that all players correctly get priority.
    pub fn start_priority_round(&mut self, player: PlayerId) {
        self.players_that_have_passed.clear();
        self.give_priority(player);
    }

    pub fn priority_player(&self) -> Option<PlayerId> {
        match &self.state {
            GameState::Player { player, action } if action == &PlayerActionCategory::Priority => {
                Some(*player)
            }
            _ => None,
        }
    }

    pub fn timestamp(&mut self) -> u64 {
        let timestamp = self.next_timestamp;
        self.next_timestamp += 1;
        timestamp
    }

    /// 704. State-Based Actions (https://mtg.gamepedia.com/State-based_action)
    ///
    /// 704.3. Whenever a player would get priority (see rule 117, “Timing and
    ///        Priority”), the game checks for any of the listed conditions for
    ///        state-based actions, then performs all applicable state-based
    ///        actions simultaneously as a single event. If any state-based
    ///        actions are performed as a result of a check, the check is
    ///        repeated; otherwise all triggered abilities that are waiting to
    ///        be put on the stack are put on the stack, then the check is
    ///        repeated. Once no more state-based actions have been performed as
    ///        the result of a check and no triggered abilities are waiting to
    ///        be put on the stack, the appropriate player gets priority. This
    ///        process also occurs during the cleanup step (see rule 514),
    ///        except that if no state-based actions are performed as the result
    ///        of the step’s first check and no triggered abilities are waiting
    ///        to be put on the stack, then no player gets priority and the step
    ///        ends.
    fn apply_state_based_actions(&mut self) {
        loop {
            if !state_based_actions::apply(self) {
                break;
            }
        }
    }

    fn pass_priority(&mut self, player: PlayerId) {
        if self.priority_player() != Some(player) {
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
        let next_player = self.players.player_after(player);
        log::debug!("Player {:?} passing priority to {:?}", player, next_player);

        if self.players_that_have_passed.contains(&next_player) {
            log::debug!("All players have passed");

            self.players_that_have_passed.clear();

            let stack = &self.zones[&ZoneId::Stack];
            if stack.is_empty() {
                self.end_current_step();
            } else {
                // 608.1. Each time all players pass in succession, the spell or
                //        ability on top of the stack resolves. (See rule 609,
                //        “Effects.”)
                self.resolve_one_from_stack();
            }
        } else {
            self.give_priority(next_player);
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

        let next_player = self.players.player_after(self.active_player);
        let is_new_turn_cycle = next_player.to_u32() == 0;

        {
            for player in &mut self.players {
                player.lands_played_this_turn = 0;
            }
        }

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
                self.start_priority_round(self.active_player);

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
                self.start_priority_round(self.active_player);
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
                self.start_priority_round(self.active_player);
            }

            Step::BeginCombat => combat::enter_begin_combat(self),
            Step::DeclareAttackers => combat::enter_declare_attackers(self),
            Step::DeclareBlockers => combat::enter_declare_blockers(self),
            Step::CombatDamage => combat::enter_combat_damage(self),
            Step::EndCombat => combat::enter_end_combat(self),

            // 513. End Step
            Step::End => {
                // 513.1. The end step has no turn-based actions. Once it
                //        begins, the active player gets priority. (See rule
                //        117, “Timing and Priority.”)
                self.start_priority_round(self.active_player);
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
                let mut damage_to_remove = Vec::new();
                for (entity, _damage) in self.world.query_mut::<(&Damage,)>() {
                    damage_to_remove.push(entity);
                }

                for entity in damage_to_remove {
                    let _ = self.world.remove_one::<Damage>(entity);
                }

                let mut to_despawn = Vec::new();
                for (entity, _effect) in self.world.query_mut::<(&UntilEotEffect,)>() {
                    to_despawn.push(entity);
                }

                for entity in to_despawn {
                    let _ = self.world.despawn(entity);
                }

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
                self.apply_state_based_actions();

                // TODO: Put stuff onto the stack, give priority if there was
                // anything.

                self.end_current_step();
            }
        }
    }

    fn resolve_one_from_stack(&mut self) {
        let stack = self.zone(ZoneId::Stack).unwrap();
        let top = match stack.members().last() {
            Some(top) => *top,
            None => return,
        };

        let object = match self.world.get::<Object>(top) {
            Ok(object) => object,
            Err(_) => {
                log::error!("Entity {:?} on stack is not an Object", top);
                return;
            }
        };

        // 608.3. If the object that’s resolving is a permanent spell, its
        //        resolution involves a single step (unless it’s an Aura, a copy
        //        of a permanent spell, or a mutating creature spell). The spell
        //        card becomes a permanent and is put onto the battlefield under
        //        the control of the spell’s controller.
        if object.types.contains(&CardType::Creature) {
            drop(object);
            self.move_object_to_zone(top, ZoneId::Battlefield);
        } else {
            // Pull some data out of this object before dropping the borrow so
            // that we can throw it in the graveyard.
            let owner = object.owner;

            log::warn!(
                "We don't know how to resolve this kind of object yet: {:?}",
                object.types
            );
            drop(object);
            self.move_object_to_zone(top, ZoneId::Graveyard(owner));
        }

        self.start_priority_round(self.active_player);
    }

    /// Marks the given player as having lost.
    fn player_loses(&mut self, player: PlayerId) {
        // TODO: Support >2 players
        let other_player = self.players.iter().find(|p| p.id != player).unwrap();

        self.state = GameState::Complete(GameOutcome::Win {
            winner: other_player.id,
        });
    }

    /// Returns the next step if there are steps to take still in this turn.
    fn next_step(&self) -> Option<Step> {
        match self.step {
            Step::Untap => Some(Step::Upkeep),
            Step::Upkeep => {
                if self.turn_number == 1 && self.active_player.to_u32() == 0 {
                    return Some(Step::Main1);
                }

                Some(Step::Draw)
            }
            Step::Draw => Some(Step::Main1),
            Step::Main1 => Some(Step::BeginCombat),
            Step::BeginCombat => Some(Step::DeclareAttackers),
            Step::DeclareAttackers => Some(Step::DeclareBlockers),
            Step::DeclareBlockers => Some(Step::CombatDamage),
            Step::CombatDamage => Some(Step::EndCombat),
            Step::EndCombat => Some(Step::Main2),
            Step::Main2 => Some(Step::End),
            Step::End => Some(Step::Cleanup),
            Step::Cleanup => None,
        }
    }

    fn play_land(&mut self, player: PlayerId, land: Entity) {
        // 116.2a Playing a land is a special action. To play a land, a player
        //        puts that land onto the battlefield from the zone it was in
        //        (usually that player’s hand). By default, a player can take
        //        this action only once during each of their turns. A player can
        //        take this action any time they have priority and the stack is
        //        empty during a main phase of their turn. See rule 305,
        //        “Lands.”

        fn inner(game: &mut Game, player: PlayerId, land: Entity) -> Result<(), String> {
            if game.active_player != player {
                return Err("it is not their turn".to_owned());
            }

            if game.step != Step::Main1 && game.step != Step::Main2 {
                return Err("it is not the main phase".to_owned());
            }

            if game.priority_player() != Some(player) {
                return Err("they do not have priority".to_owned());
            }

            let stack = game.zone(ZoneId::Stack).unwrap();
            if !stack.is_empty() {
                return Err("the stack is not empty".to_owned());
            }

            {
                let mut player_object = game
                    .players
                    .get_mut(player)
                    .ok_or_else(|| format!("{:?} is not a player", player))?;

                if player_object.lands_played_this_turn > 0 {
                    return Err("they have already played a land this turn".to_owned());
                }
                player_object.lands_played_this_turn += 1;
            }

            game.move_object_to_zone(land, ZoneId::Battlefield);

            Ok(())
        }

        if let Err(err) = inner(self, player, land) {
            log::error!("Player {:?} cannot play land {:?}: {}", player, land, err);
        }
    }
}

impl Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Turn: #{}   AP: {:?}   Step: {:?}   State: {:?}",
            self.turn_number, self.active_player, self.step, self.state
        )
    }
}

/// 500.1. A turn consists of five phases, in this order: beginning, precombat
///        main, combat, postcombat main, and ending. Each of these phases takes
///        place every turn, even if nothing happens during the phase. The
///        beginning, combat, and ending phases are further broken down into
///        steps, which proceed in order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
    EndCombat,

    // 512.1. The ending phase consists of two steps: end and cleanup.
    End,
    Cleanup,
}
