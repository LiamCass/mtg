// src/object/deck.rs
use uuid::Uuid;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum DeckId { Id(Uuid) }