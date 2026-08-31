// ante.rs
use uuid::Uuid;
use std::collections::HashSet;

/// Rule 407: the ante zone is shared by all players. Used only by
/// older/casual cards; not part of tournament-legal play.
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum AnteId { Id(Uuid) }

/// The closed set of object kinds that can occupy the Ante zone.
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum AnteContent {
    Card(CardId),
}

/// The ante zone (rule 407): a public, shared zone.
pub struct Ante {
    pub id: AnteId,
    pub contents: HashSet<AnteContent>,
}
