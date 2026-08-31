// src/zone/ante.rs
use uuid::Uuid;
use std::collections::HashSet;
use crate::object::card::CardId;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum AnteId { Id(Uuid) }

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum AnteContent {
    Card(CardId),
}

pub struct Ante {
    pub id: AnteId,
    pub contents: HashSet<AnteContent>,
}