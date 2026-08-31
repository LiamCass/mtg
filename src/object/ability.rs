// src/object/ability.rs
use uuid::Uuid;

use crate::object::card::CardId;
use crate::player::PlayerId;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum AbilityId { Id(Uuid) }

/// A minimal placeholder for an activated/triggered ability as an object
/// on the stack (rule 113, 608) — just enough to unblock
/// `StackContent::Ability`. Real ability representation (effect text,
/// cost, trigger condition, etc.) is deferred to the `mtg-abilities`
/// design work.
///
/// Known gap: `source` is typed as `CardId`, covering the common case,
/// but abilities can also originate from tokens or emblems (rule 113.7).
/// This will likely need to become a small closed enum (mirroring the
/// zone `Content` pattern) once ability representation is designed for
/// real.
#[derive(Debug, Clone)]
pub struct Ability {
    pub id: AbilityId,
    pub controller: PlayerId,
    pub source: CardId,
}