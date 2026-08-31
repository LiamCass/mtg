// src/zone/command.rs
use uuid::Uuid;
use std::collections::HashSet;
use crate::object::card::CardId;
use crate::object::emblem::EmblemId;
use crate::object::deck::DeckId;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum CommandId { Id(Uuid) }

/// Junkyard variant intentionally omitted — its zone-vs-sub-area location
/// and per-player-within-a-shared-zone shape are still unresolved.
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum CommandContent {
    Card(CardId),
    Emblem(EmblemId),
    Deck(DeckId),
}

pub struct Command {
    pub id: CommandId,
    pub contents: HashSet<CommandContent>,
}