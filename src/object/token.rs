// src/object/token.rs
use uuid::Uuid;

use crate::object::card::Characteristics;
use crate::player::PlayerId;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum TokenId { Id(Uuid) }

/// A token (rule 111): a marker representing a permanent that isn't
/// represented by a card. Reuses `Characteristics` since a token has the
/// same shape of copiable values as a card (name, color, type line,
/// P/T, etc.) — it just typically has no mana cost and no owner-printed
/// card behind it.
///
/// Known gap (rule 111.7): a token that leaves the battlefield ceases to
/// exist — that's a state-based action, not something this "dumb data"
/// crate enforces. `mtg-engine` will need to check for tokens sitting in
/// non-battlefield zones and remove them.
#[derive(Debug, Clone)]
pub struct Token {
    pub id: TokenId,
    pub owner: PlayerId,
    pub characteristics: Characteristics,
}