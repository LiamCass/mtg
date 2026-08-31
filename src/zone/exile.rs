// src/zone/exile.rs
use uuid::Uuid;
use std::collections::HashSet;
use crate::object::card::CardId;
use crate::object::token::TokenId;
use crate::object::copy::CopyId;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum ExileId { Id(Uuid) }

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum ExileContent {
    Card(CardId),
    Token(TokenId),
    Copy(CopyId),
}

pub struct Exile {
    pub id: ExileId,
    pub contents: HashSet<ExileContent>,
}