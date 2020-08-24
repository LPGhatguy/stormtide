//! Describes an action that a player can (potentially) take in the game.

#[derive(Debug)]
pub enum Action {
    /// 104.3a A player can concede the game at any time. A player who concedes
    ///        leaves the game immediately. That player loses the game.
    Concede,

    PassPriority,
}
