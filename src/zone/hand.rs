// hand.rs
use uuid::Uuid;
use std::collections::HashSet;

/// Rule 402: each player has their own hand.
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum HandId { Id(Uuid) }

/// The closed set of object kinds that can occupy the Hand zone.
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum HandContent {
    Card(CardId),
    Token(TokenId),
    Copy(CopyId),
}

/// A player's hand (rule 402): a hidden, per-player zone.
pub struct Hand {
    pub id: HandId,
    pub contents: HashSet<HandContent>,
}
