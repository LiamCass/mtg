// src/zone/library.rs
use uuid::Uuid;
use crate::object::card::CardId;
use crate::object::token::TokenId;
use crate::object::copy::CopyId;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum LibraryId { Id(Uuid) }

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum LibraryContent {
    Card(CardId),
    Token(TokenId),
    Copy(CopyId),
}

pub struct Library {
    pub id: LibraryId,
    pub contents: Vec<LibraryContent>,
}