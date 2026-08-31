// src/zone/battlefield.rs
use uuid::Uuid;
use std::collections::HashSet;
use crate::object::card::CardId;
use crate::object::token::TokenId;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum BattlefieldId { Id(Uuid) }

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum BattlefieldContent {
    Card(CardId),
    Token(TokenId),
}

pub struct Battlefield {
    pub id: BattlefieldId,
    pub contents: HashSet<BattlefieldContent>,
}