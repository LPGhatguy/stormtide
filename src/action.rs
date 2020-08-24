//! Describes an action that a player can (potentially) take in the game.

use hecs::Entity;

#[allow(unused)]
#[derive(Debug)]
pub enum Action {
    /// 104.3a A player can concede the game at any time. A player who concedes
    ///        leaves the game immediately. That player loses the game.
    Concede,

    PassPriority,

    /// 117.1a A player may cast an instant spell any time they have priority. A
    ///        player may cast a noninstant spell during their main phase any
    ///        time they have priority and the stack is empty.
    CastSpell {
        spell: Entity,
        // TODO: Targets
        // TODO: Costs
    },

    /// 117.1b A player may activate an activated ability any time they have
    ///        priority.
    ActivateAbility {
        object: Entity,
        ability: usize,
        // TODO: Targets
        // TODO: Costs
    },

    /// 116.2a Playing a land is a special action. To play a land, a player puts
    ///        that land onto the battlefield from the zone it was in (usually
    ///        that player’s hand). By default, a player can take this action
    ///        only once during each of their turns. A player can take this
    ///        action any time they have priority and the stack is empty during
    ///        a main phase of their turn. See rule 305, “Lands.”
    PlayLand {
        card: Entity,
    },
    // TODO: Rules 116.2b—116.2i
}
