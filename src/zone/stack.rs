// src/zone/stack.rs
use uuid::Uuid;
use crate::object::card::CardId;
use crate::object::copy::CopyId;
use crate::object::ability::AbilityId;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum StackId { Id(Uuid) }

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum StackContent {
    Card(CardId),
    Copy(CopyId),
    Ability(AbilityId),
}

pub struct Stack {
    pub id: StackId,
    pub contents: Vec<StackContent>,
}