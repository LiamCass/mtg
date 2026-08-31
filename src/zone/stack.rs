// stack.rs
use uuid::Uuid;

/// Uniquely identifies the stack instance in the game.
/// Rule 405: the stack is shared by all players
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum StackId { Id(Uuid) }

/// The closed set of object kinds that can occupy the Stack zone.
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum StackContent {
    Card(CardId),
    Copy(CopyId),
    Ability(AbilityId),
}

/// The stack (rule 405): a public, shared, strictly LIFO zone.
pub struct Stack {
    pub id: StackId,
    pub contents: Vec<StackContent>,
}
