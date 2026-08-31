// src/object/card.rs
use uuid::Uuid;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum CardId { Id(Uuid) }

/// One of the five colors defined by rule 105.1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}