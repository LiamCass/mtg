use std::collections::HashSet;

/*Game formats
Options
- Range of Influence always tracked per player. defaults to inf
- Attackable players always tracked per player. defaults to all
- Teams always on. Defaults to team of 1 ("You")
- Deploy Creatures ability only available when option selected
- Shared turns option managed by who can use a turn/step.

806. Free-for-All Variant
807. Grand Melee Variant
808. Team vs. Team Variant
809. Emperor Variant
810. Two-Headed Giant Variant
811. Alternating Teams Variant
901. Planechase
902. Vanguard
903. Commander
904. Archenemy
905. Conspiracy Draft
*/

impl Game {
    // 103. Starting the Game
    pub fn setup(&mut self) {
        self.setup_turn_order();      // 103.1a-c
        self.setup_starting_deck();   // 103.2a
        self.setup_companions();      // 103.2b
        self.setup_commander();       // 103.2c
        self.setup_stickers();        // 103.2d
        self.setup_conspiracy();      // 103.2e
        self.setup_library();         // 103.3
        self.setup_life_totals();     // 103.4a-e
        self.setup_starting_hand();   // 103.5
        self.starting_hand_actions(); // 103.6
        self.setup_planechase();      // 103.7
        self.setup_first_turn();      // 103.8
    }

    /// 103.1a-c: Determine beginning turn order, and starting player/team,
    /// according to cards and format.
    fn setup_turn_order(&mut self) { todo!() }

    /// 103.2a: Each player's starting deck becomes their library.
    fn setup_starting_deck(&mut self) { todo!() }

    /// 103.2b: Companion — a player may reveal a companion from outside the
    /// game and put it into the sideboard.
    fn setup_companions(&mut self) { todo!() }

    /// 103.2c: Commander — each player's commander(s) begin the game in the
    /// command zone instead of the library.
    fn setup_commander(&mut self) { todo!() }

    /// 103.2d: Stickers — a player may open sticker sheets and affix stickers
    /// to their cards before the game begins.
    fn setup_stickers(&mut self) { todo!() }

    /// 103.2e: Conspiracy Draft — conspiracy cards begin the game in the
    /// command zone.
    fn setup_conspiracy(&mut self) { todo!() }

    /// 103.3: Decks (libraries) are shuffled.
    fn setup_library(&mut self) { todo!() }

    /// 103.4a-e: Players' life totals are set according to cards and format.
    fn setup_life_totals(&mut self) { todo!() }

    /// 103.5: Starting hands and mulligans, according to cards and format.
    /// Mulligans are simultaneous choices (see 101.4).
    fn setup_starting_hand(&mut self) { todo!() }

    /// 103.6a-c: Cards that take effect prior to the game beginning.
    fn starting_hand_actions(&mut self) { todo!() }

    /// 103.7: Planechase — starting player reveals the top card of their
    /// planar deck to set the starting plane.
    fn setup_planechase(&mut self) { todo!() }

    /// 103.8, 103.8a-c: The game starts; the initial draw step is present or
    /// skipped according to cards, format, and number of players.
    fn setup_first_turn(&mut self) { todo!() }
}