// battlefield.rs
use uuid::Uuid;
use std::collections::HashSet;

/// Uniquely identifies the battlefield instance in the game.
/// Rule 403: the battlefield is shared by all players
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum BattlefieldId { Id(Uuid) }

/// The closed set of object kinds that can occupy the Battlefield zone.
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum BattlefieldContent {
    Card(CardId),
    Token(TokenId),
}

/// The battlefield (rule 403): a public, shared zone.
pub struct Battlefield {
    pub id: BattlefieldId,
    pub contents: HashSet<BattlefieldContent>,
}
