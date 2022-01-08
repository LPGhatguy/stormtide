use hecs::Entity;
use serde::{Deserialize, Serialize};

use crate::mana_pool::ManaId;

/// Describes an action that a player can take in the game.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PlayerAction {
    /// 104.3a A player can concede the game at any time. A player who concedes
    ///        leaves the game immediately. That player loses the game.
    Concede,

    PassPriority,

    ChooseAttackers {
        attackers: Vec<Entity>,
    },

    ChooseBlockers {
        blockers: Vec<Entity>,
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

    /// 117.1a A player may cast an instant spell any time they have priority. A
    ///        player may cast a noninstant spell during their main phase any
    ///        time they have priority and the stack is empty.
    StartCastingSpell {
        spell: Entity,
    },

    /// Pay part of the mana cost of an incomplete spell.
    PayIncompleteSpellMana {
        spell: Entity,
        mana: ManaId,
    },

    FinishCastingSpell {
        spell: Entity,
    },
    CancelCastingSpell {
        spell: Entity,
    },
    // 117.1b A player may activate an activated ability any time they have
    //        priority.
    // TODO: Rules 116.2b—116.2i
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlayerActionCategory {
    /// A player has priority and can start to take an action. In some steps
    /// like the untap and cleanup steps, players do not normally receive
    /// priority.
    Priority,

    ChooseAttackers,
    ChooseBlockers,

    /// A player is in the window where they can activate mana abilities to pay
    /// for a spell.
    ///
    /// 601.2g If the total cost includes a mana payment, the player then has a
    ///        chance to activate mana abilities (see rule 605, “Mana
    ///        Abilities”). Mana abilities must be activated before costs are
    ///        paid.
    SpellManaAbilities,

    /// A player is paying the costs for a spell.
    SpellPayingCost,
}
