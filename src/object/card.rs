// src/object/card.rs
use uuid::Uuid;

use crate::player::PlayerId;

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

/// Rule 205.4a supertypes. Small, closed, rarely changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Supertype {
    Basic,
    Legendary,
    Ongoing,
    Snow,
    World,
}

/// Rule 205.2a card types. Small, closed set defined by the rules
/// (new types are rare — Battle was the last addition).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardType {
    Artifact,
    Battle,
    Conspiracy,
    Creature,
    Dungeon,
    Enchantment,
    Instant,
    Kindred,
    Land,
    Phenomenon,
    Plane,
    Planeswalker,
    Scheme,
    Sorcery,
    Vanguard,
}

/// Rule 205: a card's full type line. Subtypes are open text — there are
/// hundreds of them across all categories (creature/land/artifact/etc.)
/// and new ones are printed every set, so a closed enum isn't practical.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeLine {
    pub supertypes: Vec<Supertype>,
    pub card_types: Vec<CardType>,
    pub subtypes: Vec<String>,
}

/// Rule 107.3: the variable symbols. X is by far the most common; Y and Z
/// exist on a handful of cards that use more than one variable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Variable {
    X,
    Y,
    Z,
}

/// Rule 107.4: the full mana symbol taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ManaSymbol {
    Generic(u32),                  // {0} {1} {2} ...
    Colored(Color),                // {W} {U} {B} {R} {G}
    Colorless,                     // {C}
    Snow,                          // {S}
    Variable(Variable),            // {X} {Y} {Z}
    Hybrid(Color, Color),          // {W/U}
    MonoHybrid(Color),             // {2/W} — generic or this color
    Phyrexian(Color),              // {W/P} — this color or 2 life
    HybridPhyrexian(Color, Color), // {G/W/P} — either color or 2 life
}

/// Rule 107: a mana cost is an ordered multiset of symbols.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ManaCost {
    pub symbols: Vec<ManaSymbol>,
}

/// The copiable values a card (or card-like object) currently has —
/// name, mana cost, color, type line, and the numeric characteristics
/// that only some types use. Shared shape for Card today; Token and Copy
/// will reuse this rather than duplicating it.
///
/// Known gap: power/toughness/loyalty/defense are `Option<i32>`, which
/// can't yet represent a characteristic-defining `*` (e.g. Tarmogoyf's
/// power/toughness). That needs the rule 613 layers system, which this
/// crate isn't implementing yet — this is just the data slot for it.
///
/// Known gap: no rules-text/abilities field yet — deferred until
/// `mtg-abilities` settles how abilities are represented.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Characteristics {
    pub name: String,
    pub mana_cost: ManaCost,
    pub color: Vec<Color>,
    pub type_line: TypeLine,
    pub power: Option<i32>,
    pub toughness: Option<i32>,
    pub loyalty: Option<i32>,
    pub defense: Option<i32>,
}

/// A card (rule 108): a physical/digital card with an owner and a
/// current set of characteristics. `owner` is the one field Ante (407)
/// actually mutates during play — see rule 407.3.
#[derive(Debug, Clone)]
pub struct Card {
    pub id: CardId,
    pub owner: PlayerId,
    pub characteristics: Characteristics,
}