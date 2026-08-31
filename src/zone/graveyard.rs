// src/zone/graveyard.rs
use uuid::Uuid;
use crate::object::card::CardId;
use crate::object::token::TokenId;
use crate::object::copy::CopyId;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum GraveyardId { Id(Uuid) }

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum GraveyardContent {
    Card(CardId),
    Token(TokenId),
    Copy(CopyId),
}

pub struct Graveyard {
    pub id: GraveyardId,
    pub contents: Vec<GraveyardContent>,
}