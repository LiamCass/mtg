// player.rs
use std::collections::HashMap;
use uuid::Uuid;

use crate::card::Color;
use crate::turn::TurnId;
use crate::library::LibraryId;
use crate::hand::HandId;
use crate::graveyard::GraveyardId;
use crate::exile::ExileId;
use crate::command::CommandId;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum PlayerId { Id(Uuid) }

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum TeamId { Id(Uuid) }

// Rule 104 — Game Outcomes
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

    // Active Player & Shared Teams Turns
    pub active: Option<TurnId>,

    // Options & Variants (801) — !0 (u32::MAX) means unrestricted; reduced only when a limited-range effect applies
    pub influence: u32,

    // Turn Rules & Constraints (Rule 305 & 402)
    pub max_lands_per_turn: u32,
    pub max_hand_size: u32,

    // Player modifiers
    pub life_total: i32,
    pub counters: HashMap<String, u32>,
    pub mana_pool: HashMap<Option<Color>, u32>, // 106.4

    // Player Owned Zone IDs (Rule 400)
    pub library: LibraryId,
    pub hand: HandId,
    pub graveyard: GraveyardId,
    pub exile: ExileId,
    pub command: CommandId,
}