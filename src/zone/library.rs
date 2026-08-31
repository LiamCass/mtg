// library.rs
use uuid::Uuid;

/// Uniquely identifies a specific library instance in the game.
/// Rule 401: each player has their own library.
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum LibraryId { Id(Uuid) }

/// The closed set of object kinds that can occupy the Library zone.
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum LibraryContent {
    Card(CardId),
    Token(TokenId),
    Copy(CopyId),
}

/// A player's library (rule 401): a hidden, per-player, strictly ordered zone.
pub struct Library {
    pub id: LibraryId,
    pub contents: Vec<LibraryContent>,
}
