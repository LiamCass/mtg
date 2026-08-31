// command.rs
use uuid::Uuid;
use std::collections::HashSet;

/// Uniquely identifies the command instance in the game.
/// Rule 408: the command zone is shared by all players.
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum CommandId { Id(Uuid) }

/// The closed set of object kinds that can occupy the Command zone.
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum CommandContent {
    Card(CardId),
    Emblem(EmblemId),
    Deck(DeckId),
    Junkyard(JunkyardId),
}

/// The command zone (rule 408): a public, shared zone.
pub struct Command {
    pub id: CommandId,
    pub contents: HashSet<CommandContent>,
}
