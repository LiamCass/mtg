use uuid::Uuid;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum PhaseKind {
    Beginning, // 501
    Main,      // 505
    Combat,    // 506
    Ending,    // 512
}

// rule 500.1 — StepKind
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum StepKind {
    // Beginning phase (501)
    Untap,  // 502
    Upkeep, // 503
    Draw,   // 504

    // Combat phase (506)
    BeginningOfCombat, // 507
    DeclareAttackers,  // 508
    DeclareBlockers,   // 509
    CombatDamage,      // 510
    EndOfCombat,       // 511

    // Ending phase (512)
    End,     // 513
    Cleanup, // 514
}

// Wrappers for identifying the ID
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum StepId { Id(Uuid),}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum PhaseId { Id(Uuid),}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum TurnId { Id(Uuid),}

// identifies one Step instance.
#[derive(Clone, Debug)]
pub struct Step {
    pub id: StepId,
    pub kind: StepKind,
    pub skipped: bool,
}

// identifies one Phase instance.
#[derive(Clone, Debug)]
pub struct Phase {
    pub id: PhaseId,
    pub kind: PhaseKind,
    pub skipped: bool,
    pub steps: Vec<Step>,
    pub current: usize,
}

// identifies one Turn instance.
#[derive(Debug, Clone)]
pub struct Turn {
    pub id: TurnId,
    pub phases: Vec<Phase>,
    pub current: usize,
}