// src/object/emblem.rs
use uuid::Uuid;

use crate::player::PlayerId;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum EmblemId { Id(Uuid) }

/// An emblem (rule 111): a marker representing a set of one or more
/// abilities. Unlike Card/Token/Copy, an emblem has no characteristics
/// at all — it isn't a card, permanent, or token. It's created into the
/// command zone and stays there for the rest of the game.
///
/// Tracked by controller (the player it was created for), matching the
/// same reasoning as `Copy` — an emblem isn't owned in the print-history
/// sense a Card is.
///
/// Known gap: no abilities field yet. An emblem's entire purpose is to
/// grant static abilities, so this is a placeholder until `mtg-abilities`
/// settles ability representation — at that point this will likely need
/// something like `abilities: Vec<AbilityId>`.
#[derive(Debug, Clone)]
pub struct Emblem {
    pub id: EmblemId,
    pub controller: PlayerId,
}