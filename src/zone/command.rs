// src/zone/command.rs
use uuid::Uuid;
use std::collections::HashSet;
use crate::object::card::CardId;
use crate::object::emblem::EmblemId;
use crate::object::deck::DeckId;
use crate::object::junkyard::JunkyardId;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum CommandId { Id(Uuid) }

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum CommandContent {
    Card(CardId),
    Emblem(EmblemId),
    Deck(DeckId),
    Junkyard(JunkyardId),
}

/// The command zone (rule 408). Shared, single instance. Multiple
/// players' Decks and Junkyards can coexist in the same `contents` set
/// — each carries its own `owner` field to disambiguate whose is whose,
/// rather than Command needing any per-player structure of its own.
pub struct Command {
    pub id: CommandId,
    pub contents: HashSet<CommandContent>,
}