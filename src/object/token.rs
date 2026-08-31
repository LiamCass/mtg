// src/object/token.rs
use uuid::Uuid;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum TokenId { Id(Uuid) }