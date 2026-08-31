// graveyard.rs
use uuid::Uuid;

/// Uniquely identifies a specific graveyard instance in the game.
/// Rule 404: each player has their own graveyard.
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum GraveyardId { Id(Uuid) }

/// The closed set of object kinds that can occupy the Graveyard zone.
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum GraveyardContent {
    Card(CardId),
    Token(TokenId),
    Copy(CopyId),
}

/// A player's graveyard (rule 404): a public, per-player, ordered pile.
/// Simultaneous additions. may be arranged freely by their owner (rule 404.3).
pub struct Graveyard {
    pub id: GraveyardId,
    pub contents: Vec<GraveyardContent>,
}
