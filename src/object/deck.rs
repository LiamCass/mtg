// src/object/deck.rs
use uuid::Uuid;

use crate::object::card::CardId;
use crate::player::PlayerId;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum DeckId { Id(Uuid) }

/// A supplementary deck (rules 901/904 — planar deck, scheme deck; the
/// same shape covers attraction decks too). Explicitly distinct from
/// `Library`: rule 401.1 vs. 100.2d — a supplementary deck lives in the
/// command zone and never becomes a player's library, no matter how
/// similar the shape looks.
///
/// Ordered, top = index 0 — mirrors the convention already settled for
/// `Library.contents`, for consistency between the two ordered-pile
/// types.
///
/// `owner` breaks from the `Library` pattern on purpose: `Library` has
/// no owner field because each player holds a dedicated `LibraryId`
/// directly. A `Deck` instead lives inside the single *shared* `Command`
/// zone (see `CommandContent::Deck`), and multiple players can each have
/// their own planar/scheme deck sitting in that same shared zone at
/// once (e.g. multiplayer Planechase) — there's no per-player slot on
/// `Player` to disambiguate whose is whose, so `owner` has to live here.
#[derive(Debug, Clone)]
pub struct Deck {
    pub id: DeckId,
    pub owner: PlayerId,
    pub contents: Vec<CardId>,
}