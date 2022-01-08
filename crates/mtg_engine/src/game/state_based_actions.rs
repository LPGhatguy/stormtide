use crate::components::{AttachedToEntity, Player};

use super::Game;

pub fn apply(game: &mut Game) -> bool {
    let mut actions_performed = false;

    // Clear any effects attached to objects that no longer exist.
    {
        let mut entities_to_despawn = Vec::new();
        let mut query = game.world.query::<(&AttachedToEntity,)>();

        // Should this be applied recursively? For simplicity, it is not,
        // but this means that nested chains of attachments may not be
        // removed correctly.
        for (entity, (attached,)) in query.iter() {
            if !game.world.contains(attached.target) {
                entities_to_despawn.push(entity);
                actions_performed = true;
            }
        }

        drop(query);

        for entity in entities_to_despawn {
            game.world.despawn(entity).unwrap();
        }
    }

    // 704.5a If a player has 0 or less life, that player loses the game.
    {
        let mut player_query = game.world.query::<(&mut Player,)>();

        for (_entity, (player,)) in player_query.iter() {
            if !player.has_lost && player.life <= 0 {
                // TODO: Check if player is exempt from this SBA, like via
                // Phyrexian Unlife.
                player.has_lost = true;
                actions_performed = true;
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

    actions_performed
}
