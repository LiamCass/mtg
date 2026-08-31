// src/object/emblem.rs
use uuid::Uuid;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum EmblemId { Id(Uuid) }