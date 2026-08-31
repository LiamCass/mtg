// exile.rs
use uuid::Uuid;
use std::collections::HashSet;

/// Uniquely identifies the exile instance in the game.
/// Rule 406: the exile zone is shared by all players.
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum ExileId { Id(Uuid) }

/// The closed set of object kinds that can occupy the Exile zone.
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum ExileContent {
    Card(CardId),
    Token(TokenId),
    Copy(CopyId),
}

/// The exile zone (rule 406): a public, shared holding area.
pub struct Exile {
    pub id: ExileId,
    pub contents: HashSet<ExileContent>,
}
