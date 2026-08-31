// src/zone/hand.rs
use uuid::Uuid;
use std::collections::HashSet;
use crate::object::card::CardId;
use crate::object::token::TokenId;
use crate::object::copy::CopyId;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum HandId { Id(Uuid) }

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum HandContent {
    Card(CardId),
    Token(TokenId),
    Copy(CopyId),
}

pub struct Hand {
    pub id: HandId,
    pub contents: HashSet<HandContent>,
}