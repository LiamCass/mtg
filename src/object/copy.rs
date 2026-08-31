// src/object/copy.rs
use uuid::Uuid;

use crate::object::card::Characteristics;
use crate::player::PlayerId;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum CopyId { Id(Uuid) }

/// A copy (rule 706): the result of copying a spell or ability onto the
/// stack. A copy has no owner in the usual sense — rule 706.10 has it
/// cease to exist as a state-based action once it leaves the stack (a
/// copy of a permanent spell becomes a token permanent instead, handled
/// by `Token`, not this type). It's tracked by controller, not owner.
///
/// Reuses `Characteristics`, acquired from the copied object per rule
/// 707.2 — the *copiable* values only, not counters, stickers, or other
/// effects layered on top of the original.
///
/// Known gap / flag for the later rework pass: per rule 706.10, a Copy
/// should only ever legally exist on the Stack — but `CopyId` currently
/// also appears in Hand/Library/Graveyard/Exile's content unions from
/// earlier zone design work. Worth revisiting whether those are correct
/// or should be narrowed once we do a full pass.
#[derive(Debug, Clone)]
pub struct Copy {
    pub id: CopyId,
    pub controller: PlayerId,
    pub characteristics: Characteristics,
}