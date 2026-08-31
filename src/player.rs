// src/player.rs
use std::collections::HashMap;
use uuid::Uuid;

use crate::object::card::Color;
use crate::turn::TurnId;
use crate::zone::library::LibraryId;
use crate::zone::hand::HandId;
use crate::zone::graveyard::GraveyardId;
use crate::zone::exile::ExileId;
use crate::zone::command::CommandId;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum PlayerId { Id(Uuid) }

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum TeamId { Id(Uuid) }

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Outcome {
    Won,
    Lost,
    Drawn,
    Playing,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub id: PlayerId,
    pub team: TeamId,
    pub status: Outcome,
    pub active: Option<TurnId>,
    pub influence: u32,
    pub max_lands_per_turn: u32,
    pub max_hand_size: u32,
    pub life_total: i32,
    pub counters: HashMap<String, u32>,
    pub mana_pool: HashMap<Option<Color>, u32>,
    pub library: LibraryId,
    pub hand: HandId,
    pub graveyard: GraveyardId,
    pub exile: ExileId,
    pub command: CommandId,
}