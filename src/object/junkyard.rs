// src/object/junkyard.rs
use uuid::Uuid;
use std::collections::HashSet;

use crate::object::card::CardId;
use crate::player::PlayerId;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum JunkyardId { Id(Uuid) }

/// A player's junkyard (rule 718.6a): a face-up pile of Attraction cards
/// that have left the battlefield, kept separate from that player's
/// Attraction deck. Given its own identity to match every other
/// object/zone in this crate — but rule 718.6a is explicit that "the
/// pile is not its own zone," so this deliberately mirrors `Deck`
/// rather than the real eight zones: it lives in `object/`, not `zone/`,
/// and is referenced from `Command`'s shared `contents` (see
/// `CommandContent::Junkyard`) with its own `owner` field to
/// disambiguate whose pile is whose, exactly like `Deck` already does.
///
/// Cards here aren't type-checked as Attractions at this layer (subtypes
/// are open `String`s, not a closed enum — see `TypeLine`), so nothing
/// stops a non-Attraction `CardId` from being inserted. Enforcing that
/// is rules behavior (`mtg-engine`'s job), not a data-shape concern.
///
/// Known gap: rule 718.6a doesn't specify the junkyard as ordered, so
/// this uses `HashSet`. Revisit if an effect referencing junkyard order
/// turns up later.
#[derive(Debug, Clone)]
pub struct Junkyard {
    pub id: JunkyardId,
    pub owner: PlayerId,
    pub contents: HashSet<CardId>,
}