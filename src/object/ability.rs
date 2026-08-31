// src/object/ability.rs
use uuid::Uuid;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum AbilityId { Id(Uuid) }