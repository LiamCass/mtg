// src/object/copy.rs
use uuid::Uuid;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum CopyId { Id(Uuid) }