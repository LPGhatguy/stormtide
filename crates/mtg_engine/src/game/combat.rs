use hecs::Entity;

use crate::action::PlayerActionCategory;
use crate::components::Object;
use crate::types::CardType;
use crate::{components::Permanent, player::PlayerId};

use super::{Game, GameState};

/// 507. Beginning of Combat Step
pub fn enter_begin_combat(game: &mut Game) {
    // 507.1. First, if the game being played is a multiplayer game
    //        in which the active player’s opponents don’t all
    //        automatically become defending players, the active
    //        player chooses one of their opponents. That player
    //        becomes the defending player. This turn-based action
    //        doesn’t use the stack. (See rule 506.2.)
    //
    // TODO

    // 507.2. Second, the active player gets priority. (See rule
    //        117, “Timing and Priority.”)
    game.give_priority(game.active_player);
}

/// 508. Declare Attackers Step
pub fn enter_declare_attackers(game: &mut Game) {
    // 508.1. First, the active player declares attackers. This
    //        turn-based action doesn’t use the stack. To declare
    //        attackers, the active player follows the steps below,
    //        in order. If at any point during the declaration of
    //        attackers, the active player is unable to comply with
    //        any of the steps listed below, the declaration is
    //        illegal; the game returns to the moment before the
    //        declaration (see rule 726, “Handling Illegal
    //        Actions”).
    game.state = GameState::Player {
        player: game.active_player,
        action: PlayerActionCategory::ChooseAttackers,
    };
}

/// 509. Declare Blockers Step
pub fn enter_declare_blockers(game: &mut Game) {
    // TODO: Choose player who is defender instead of just "not the
    // active player"
    let nap = game.players.player_after(game.active_player);

    // 509.1. First, the defending player declares blockers. This
    //        turn-based action doesn’t use the stack. To declare
    //        blockers, the defending player follows the steps
    //        below, in order. If at any point during the
    //        declaration of blockers, the defending player is
    //        unable to comply with any of the steps listed below,
    //        the declaration is illegal; the game returns to the
    //        moment before the declaration (see rule 726, “Handling
    //        Illegal Actions”).
    game.state = GameState::Player {
        player: nap,
        action: PlayerActionCategory::ChooseBlockers,
    };

    // 509.2. Second, for each attacking creature that’s become
    //        blocked, the active player announces that creature’s
    //        damage assignment order, which consists of the
    //        creatures blocking it in an order of that player’s
    //        choice. (During the combat damage step, an attacking
    //        creature can’t assign combat damage to a creature
    //        that’s blocking it unless each creature ahead of that
    //        blocking creature in its order is assigned lethal
    //        damage.) This turn-based action doesn’t use the stack.
    //
    // TODO

    // 509.3. Third, for each blocking creature, the defending
    //        player announces that creature’s damage assignment
    //        order, which consists of the creatures it’s blocking
    //        in an order of that player’s choice. (During the
    //        combat damage step, a blocking creature can’t assign
    //        combat damage to a creature it’s blocking unless each
    //        creature ahead of that blocked creature in its order
    //        is assigned lethal damage.) This turn-based action
    //        doesn’t use the stack.
    //
    // TODO

    // 509.4. Fourth, the active player gets priority. (See rule 117, “Timing and Priority.”)
    //
    // 509.4a Any abilities that triggered on blockers being
    //        declared or that triggered during the process
    //        described in rules 509.1–3 are put onto the stack
    //        before the active player gets priority; the order in
    //        which they triggered doesn’t matter. (See rule 603,
    //        “Handling Triggered Abilities.”)
    //
    // TODO
}

// 510. Combat Damage Step
pub fn enter_combat_damage(game: &mut Game) {
    // 510.1. First, the active player announces how each attacking
    //        creature assigns its combat damage, then the defending
    //        player announces how each blocking creature assigns
    //        its combat damage. This turn-based action doesn’t use
    //        the stack. A player assigns a creature’s combat damage
    //        according to the following rules:
    //
    // TODO

    // 510.2. Second, all combat damage that’s been assigned is
    //        dealt simultaneously. This turn-based action doesn’t
    //        use the stack. No player has the chance to cast spells
    //        or activate abilities between the time combat damage
    //        is assigned and the time it’s dealt.
    //
    // TODO

    // 510.3. Third, the active player gets priority. (See rule 117, “Timing and Priority.”)
    //
    // 510.3a Any abilities that triggered on damage being dealt or
    //        while state-based actions are performed afterward are
    //        put onto the stack before the active player gets
    //        priority; the order in which they triggered doesn’t
    //        matter. (See rule 603, “Handling Triggered
    //        Abilities.”)
    game.give_priority(game.active_player);
}

// 511. End of Combat Step
pub fn enter_end_combat(game: &mut Game) {
    // 511.1. The end of combat step has no turn-based actions. Once
    //        it begins, the active player gets priority. (See rule
    //        117, “Timing and Priority.”)
    game.give_priority(game.active_player);

    // 511.2. Abilities that trigger “at end of combat” trigger as
    //        the end of combat step begins. Effects that last
    //        “until end of combat” expire at the end of the combat
    //        phase.
    //
    // TODO

    // 511.3. As soon as the end of combat step ends, all creatures
    //        and planeswalkers are removed from combat. After the
    //        end of combat step ends, the combat phase is over and
    //        the postcombat main phase begins (see rule 505).
    //
    // TODO
}

fn attackers_valid(game: &Game, player: PlayerId, attackers: &[Entity]) -> Result<(), String> {
    // 508. Declare Attackers Step

    // 508.1a The active player chooses which creatures that they control,
    //        if any, will attack. The chosen creatures must be untapped,
    //        and each one must either have haste or have been controlled by
    //        the active player continuously since the turn began.
    for &attacker in attackers {
        let entity = game
            .world
            .entity(attacker)
            .map_err(|_| format!("Entity {:?} did not exist", attacker))?;

        let object = entity
            .get::<Object>()
            .ok_or_else(|| format!("Entity {:?} is not an Object", attacker))?;

        if object.controller != Some(player) {
            return Err(format!(
                "Entity {:?} is not controlled by {:?}",
                attacker, player
            ));
        }

        // FIXME: Use type query instead to figure out whether something is
        // a permanent.
        let permanent = entity
            .get::<Permanent>()
            .ok_or_else(|| format!("Entity {:?} is not a permanent", attacker))?;

        // FIXME: Use type query instead to figure out whether something is
        // a creature.
        if !object.types.contains(&CardType::Creature) {
            return Err(format!("Entity {:?} is not a creature", attacker));
        }

        if permanent.tapped {
            return Err(format!("Creature {:?} is tapped", attacker));
        }

        // TODO: Check for control timestamp or haste.
    }

    // 508.1b If the defending player controls any planeswalkers, or the
    //        game allows the active player to attack multiple other
    //        players, the active player announces which player or
    //        planeswalker each of the chosen creatures is attacking.
    //
    // TODO: This should be part of the declaration information.

    // 508.1c The active player checks each creature they control to see
    //        whether it’s affected by any restrictions (effects that say a
    //        creature can’t attack, or that it can’t attack unless some
    //        condition is met). If any restrictions are being disobeyed,
    //        the declaration of attackers is illegal.
    //
    // TODO
    //
    // Example card: Pacifism

    // 508.1d The active player checks each creature they control to see
    //        whether it’s affected by any requirements (effects that say a
    //        creature attacks if able, or that it attacks if some condition
    //        is met). If the number of requirements that are being obeyed
    //        is fewer than the maximum possible number of requirements that
    //        could be obeyed without disobeying any restrictions, the
    //        declaration of attackers is illegal. If a creature can’t
    //        attack unless a player pays a cost, that player is not
    //        required to pay that cost, even if attacking with that
    //        creature would increase the number of requirements being
    //        obeyed. If a requirement that says a creature attacks if able
    //        during a certain turn refers to a turn with multiple combat
    //        phases, the creature attacks if able during each declare
    //        attackers step in that turn.
    //
    // TODO
    //
    // Example card: Curse of the Nightly Hunt

    Ok(())
}

pub fn choose_attackers(game: &mut Game, player: PlayerId, attackers: &[Entity]) {
    log::info!("Player {:?} chose attackers {:?}", player, attackers);

    let required_state = GameState::Player {
        player,
        action: PlayerActionCategory::ChooseAttackers,
    };

    if game.state != required_state {
        log::warn!("Player {:?} cannot choose attackers right now.", player);
        return;
    }

    if let Err(reason) = attackers_valid(game, player, attackers) {
        log::warn!("Attackers were not valid: {}", reason);
        return;
    }

    // 508.1e If any of the chosen creatures have banding or a “bands with
    //        other” ability, the active player announces which creatures,
    //        if any, are banded with which. (See rule 702.22, “Banding.”)
    //
    // TODO

    // 508.1f The active player taps the chosen creatures. Tapping a
    //        creature when it’s declared as an attacker isn’t a cost;
    //        attacking simply causes creatures to become tapped.
    for &attacker in attackers {
        let mut permanent = game.world.get_mut::<Permanent>(attacker).unwrap();
        permanent.tapped = true;
    }

    // 508.1g If there are any optional costs to attack with the chosen
    //        creatures (expressed as costs a player may pay “as” a creature
    //        attacks), the active player chooses which, if any, they will
    //        pay.
    //
    // TODO

    // 508.1h If any of the chosen creatures require paying costs to attack,
    //        or if any optional costs to attack were chosen, the active
    //        player determines the total cost to attack. Costs may include
    //        paying mana, tapping permanents, sacrificing permanents,
    //        discarding cards, and so on. Once the total cost is
    //        determined, it becomes “locked in.” If effects would change
    //        the total cost after this time, ignore this change.
    //
    // TODO

    // 508.1i If any of the costs require mana, the active player then has a
    //        chance to activate mana abilities (see rule 605, “Mana
    //        Abilities”).
    //
    // TODO

    // 508.1j Once the player has enough mana in their mana pool, they pay
    //        all costs in any order. Partial payments are not allowed.
    //
    // TODO

    // 508.1k Each chosen creature still controlled by the active player
    //        becomes an attacking creature. It remains an attacking
    //        creature until it’s removed from combat or the combat phase
    //        ends, whichever comes first. See rule 506.4.
    //
    // TODO

    // 508.1m Any abilities that trigger on attackers being declared
    //        trigger.
    // TODO

    // 508.2. Second, the active player gets priority. (See rule 117,
    //        “Timing and Priority.”)
    game.start_priority_round(game.active_player);
}

fn blockers_valid(_game: &Game, _player: PlayerId, _blockers: &[Entity]) -> Result<(), String> {
    Ok(())
}

pub fn choose_blockers(game: &mut Game, player: PlayerId, blockers: &[Entity]) {
    let required_state = GameState::Player {
        player,
        action: PlayerActionCategory::ChooseBlockers,
    };

    if game.state != required_state {
        log::warn!("Player {:?} cannot choose blockers right now.", player);
        return;
    }

    if let Err(reason) = blockers_valid(game, player, blockers) {
        log::warn!("Blockers were not valid: {}", reason);
        return;
    }

    // TODO

    game.start_priority_round(game.active_player);
}
