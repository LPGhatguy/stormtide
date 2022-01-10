//! 601. Casting Spells

use std::collections::HashSet;

use hecs::Entity;

use crate::action::PlayerActionCategory;
use crate::components::{IncompleteSpell, Object, Player};
use crate::game::GameState;
use crate::mana_pool::ManaId;
use crate::types::CardType;
use crate::zone::ZoneId;

use super::{Game, Step};

/// 601.2. To cast a spell is to take it from where it is (usually the
///        hand), put it on the stack, and pay its costs, so that it will
///        eventually resolve and have its effect. Casting a spell includes
///        proposal of the spell (rules 601.2a–d) and determination and
///        payment of costs (rules 601.2f–h). To cast a spell, a player
///        follows the steps listed below, in order. A player must be
///        legally allowed to cast the spell to begin this process (see rule
///        601.3). If a player is unable to comply with the requirements of
///        a step listed below while performing that step, the casting of
///        the spell is illegal ; the game returns to the moment before the
///        casting of that spell was proposed (see rule 726, “Handling
///        Illegal Actions”).
pub fn start_casting_spell(game: &mut Game, player: Entity, spell: Entity) {
    let mut inner = || {
        // 601.3. A player can begin to cast a spell only if a rule or
        //        effect allows that player to cast it and no rule or effect
        //        prohibits that player from casting it.
        //
        let previous_zone = {
            let spell_object = game
                .world
                .get::<Object>(spell)
                .map_err(|_| "spell is not an Object")?;

            // 117.1a A player may cast an instant spell any time they have
            //        priority. A player may cast a noninstant spell during
            //        their main phase any time they have priority and the stack
            //        is empty.
            if game.priority_player() != Some(player) {
                return Err("player does not have priority");
            }

            if !spell_object.types.contains(&CardType::Instant) {
                if !game.zone(ZoneId::Stack).unwrap().is_empty() {
                    return Err("stack is not empty");
                }

                if game.step != Step::Main1 && game.step != Step::Main2 {
                    return Err("it is not a main phase");
                }
            }

            // By default, players can only cast spells in their hands.
            if spell_object.zone != ZoneId::Hand(player) {
                return Err("spell is not in that player's hand");
            }

            spell_object.zone
        };

        // 601.2a To propose the casting of a spell, a player first moves that
        //        card (or that copy of a card) from where it is to the stack.
        //        It becomes the topmost object on the stack. It has all the
        //        characteristics of the card (or the copy of a card)
        //        associated with it, and that player becomes its controller.
        //        The spell remains on the stack until it resolves, it’s
        //        countered, or a rule or effect moves it elsewhere.
        game.move_object_to_zone(spell, ZoneId::Stack);

        // 601.2b If the spell is modal, the player announces the mode
        //        choice (see rule 700.2).

        // 601.2c The player announces their choice of an appropriate object
        //        or player for each target the spell requires.

        // 601.2d If the spell requires the player to divide or distribute
        //        an effect (such as damage or counters) among one or more
        //        targets, the player announces the division. Each of these
        //        targets must receive at least one of whatever is being
        //        divided.

        // 601.2e The game checks to see if the proposed spell can legally
        //        be cast. If the proposed spell is illegal, the game
        //        returns to the moment before the casting of that spell was
        //        proposed (see rule 726, “Handling Illegal Actions”).

        // 601.2f The player determines the total cost of the spell.
        let total_cost = {
            // Safe because we've checked that this was a legal object.
            let spell = game.world.get::<Object>(spell).unwrap();

            match &spell.mana_cost {
                Some(mana_cost) => mana_cost.clone(),
                None => return Err("spell has no mana cost"),
            }
        };

        // 601.2g If the total cost includes a mana payment, the player then
        //        has a chance to activate mana abilities (see rule 605,
        //        “Mana Abilities”). Mana abilities must be activated before
        //        costs are paid.
        game.state = GameState::Player {
            player,
            action: PlayerActionCategory::SpellManaAbilities,
        };

        game.world
            .insert_one(spell, IncompleteSpell::new(previous_zone, total_cost))
            .unwrap();

        // ...continues in `pay_spell_mana` and `finish_casting_spell`

        Ok::<(), &str>(())
    };

    if let Err(err) = inner() {
        log::error!("Player {:?} cannot cast spell {:?}: {}", player, spell, err);
        cancel_casting_spell(game, player, spell);
    }
}

pub fn pay_spell_mana(game: &mut Game, player: Entity, spell: Entity, mana_id: ManaId) {
    let inner = || {
        let player_data = game
            .world
            .get::<Player>(player)
            .map_err(|_| "player was invalid")?;

        let spell_object = game
            .world
            .get::<Object>(spell)
            .map_err(|_| "spell is not an Object")?;

        let mut incomplete = game
            .world
            .get_mut::<IncompleteSpell>(spell)
            .map_err(|_| "spell is not an IncompleteSpell")?;

        if spell_object.controller != Some(player) {
            return Err("spell is not controlled by player");
        }

        let mana = player_data
            .mana_pool
            .get(mana_id)
            .ok_or("mana ID was invalid")?;

        if incomplete.mana_paid.contains(&mana_id) {
            return Err("mana was already spent");
        }

        // TODO: Handle X mana
        let amount_paid_so_far = incomplete.mana_paid.len();
        let next_mana = incomplete
            .total_cost
            .items
            .get(amount_paid_so_far)
            .ok_or("no more mana needs to be paid")?;

        if !next_mana.can_be_paid_with(&mana) {
            return Err("mana cost cannot be paid with that mana");
        }

        incomplete.mana_paid.push(mana_id);

        Ok::<(), &str>(())
    };

    if let Err(err) = inner() {
        log::error!(
            "Player {:?} cannot pay mana {:?} to cast spell {:?}: {}",
            player,
            mana_id,
            spell,
            err
        );
    }
}

pub fn finish_casting_spell(game: &mut Game, player: Entity, spell: Entity) {
    let mut inner = || -> Result<(), String> {
        {
            let mut player_data = game
                .world
                .get_mut::<Player>(player)
                .map_err(|_| "player is not a Player")?;

            let spell_object = game
                .world
                .get::<Object>(spell)
                .map_err(|_| "spell is not an Object")?;

            let spell_incomplete = game
                .world
                .get::<IncompleteSpell>(spell)
                .map_err(|_| "spell is not an IncompleteSpell")?;

            if spell_object.controller != Some(player) {
                return Err("spell is not controlled by player".to_owned());
            }

            let mut mana_spent = HashSet::new();

            for (i, cost) in spell_incomplete.total_cost.items.iter().enumerate() {
                let mana_id = spell_incomplete
                    .mana_paid
                    .get(i)
                    .ok_or_else(|| format!("Mana cost {} ({:?}) is not yet paid", i, cost))?;

                if mana_spent.contains(mana_id) {
                    return Err(format!("Mana {:?} was already spent", mana_id));
                }

                mana_spent.insert(*mana_id);

                let mana = player_data
                    .mana_pool
                    .get(*mana_id)
                    .ok_or_else(|| format!("Mana {:?} was not valid", mana_id))?;

                if !cost.can_be_paid_with(&mana) {
                    return Err(format!(
                        "Mana {} ({:?}) cannot be paid with {:?}",
                        i, cost, mana
                    ));
                }
            }

            // 601.2h The player pays the total cost. First, they pay all costs
            //        that don’t involve random elements or moving objects from
            //        the library to a public zone, in any order. Then they pay
            //        all remaining costs in any order. Partial payments are not
            //        allowed. Unpayable costs can’t be paid.
            player_data.mana_pool.spend(&spell_incomplete.mana_paid);
        }

        // 601.2i Once the steps described in 601.2a–h are completed,
        //        effects that modify the characteristics of the spell as
        //        it’s cast are applied, then the spell becomes cast. Any
        //        abilities that trigger when a spell is cast or put onto
        //        the stack trigger at this time. If the spell’s controller
        //        had priority before casting it, they get priority.
        //
        // TODO: Spell modifications, triggers
        game.world.remove_one::<IncompleteSpell>(spell).unwrap();
        game.start_priority_round(player);

        Ok::<(), String>(())
    };

    if let Err(err) = inner() {
        log::error!(
            "Player {:?} cannot finish casting spell {:?}: {}",
            player,
            spell,
            err
        );
    }
}

pub fn cancel_casting_spell(game: &mut Game, player: Entity, spell: Entity) {
    let mut inner = || {
        let (current_zone, previous_zone) = {
            let spell_entity = game.world.entity(spell).map_err(|_| "spell not found")?;

            let spell_object = spell_entity
                .get::<Object>()
                .ok_or("spell was not an Object")?;

            let spell_incomplete = spell_entity
                .get::<IncompleteSpell>()
                .ok_or("not an incomplete spell")?;

            (spell_object.zone, spell_incomplete.previous_zone)
        };

        game.world.remove_one::<IncompleteSpell>(spell).unwrap();

        if current_zone != previous_zone {
            game.move_object_to_zone(spell, previous_zone);
        }

        Ok::<(), &str>(())
    };

    if let Err(err) = inner() {
        log::info!(
            "Player {:?} cannot cancel casting spell {:?}: {}",
            player,
            spell,
            err
        );
    }
}
