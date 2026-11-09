//! The Produce action: union wages, then one container per factory.

use crate::action::District;
use crate::color::{Color, ColorCounts};
use crate::game::Game;
use crate::phase::{Phase, ProduceChoice};

/// Paid to the player on the right, however many containers are produced.
pub const UNION_WAGES: u32 = 1;

impl Game {
    /// Once the Bank has been called, the turn must not also end the game by producing.
    pub(crate) fn can_produce(&self) -> bool {
        !self.turn.produced
            && self.state.players[self.state.active_player].cash >= UNION_WAGES
            && (!self.turn.bank_called
                || self.can_complete_without_ending(self.production_choice()))
    }

    /// Colors the active player's factories can produce: they own the factory and the supply has a container.
    fn producible(&self) -> ColorCounts {
        let player = &self.state.players[self.state.active_player];
        let mut candidates = ColorCounts::default();
        for color in player.factory_colors() {
            if self.state.supply.containers[color] > 0 {
                candidates[color] = 1;
            }
        }
        candidates
    }

    /// Production fills the free storage room; with more candidates than room the player chooses.
    fn production_choice(&self) -> ProduceChoice {
        let candidates = self.producible();
        let room = self.state.players[self.state.active_player].factory_room();
        ProduceChoice::new(candidates, room.min(candidates.total()) as u8)
    }

    /// Whether the choice can be completed without running the supply out of enough colors to end
    /// the game.
    fn can_complete_without_ending(&self, choice: ProduceChoice) -> bool {
        if choice.to_pick == 0 {
            return !self.ends_game_after_taking(&choice.picked);
        }
        Color::ALL.into_iter().any(|color| {
            choice.can_pick(color) && self.can_complete_without_ending(choice.after(color))
        })
    }

    pub(crate) fn can_produce_color(&self, choice: &ProduceChoice, color: Color) -> bool {
        choice.can_pick(color)
            && (!self.turn.bank_called || self.can_complete_without_ending(choice.after(color)))
    }

    pub(crate) fn produce(&mut self) {
        let me = self.state.active_player;
        let right = self.state.player_on_right(me);
        self.state.players[me].cash -= UNION_WAGES;
        self.state.players[right].cash += UNION_WAGES;
        self.turn.produced = true;

        let choice = self.production_choice();
        if choice.to_pick == 0 {
            self.finish_produce(ColorCounts::default());
        } else if u32::from(choice.to_pick) == choice.candidates.total() {
            self.finish_produce(choice.candidates);
        } else {
            self.phase = Phase::Produce(choice);
        }
    }

    pub(crate) fn produce_color(&mut self, color: Color) {
        let Phase::Produce(choice) = self.phase else {
            unreachable!("produce_color outside of a color choice");
        };
        let choice = choice.after(color);
        if choice.to_pick == 0 {
            self.finish_produce(choice.picked);
        } else {
            self.phase = Phase::Produce(choice);
        }
    }

    /// Takes the produced containers from the supply and lays out the factory district again.
    fn finish_produce(&mut self, produced: ColorCounts) {
        self.state.supply.containers.sub_all(&produced);
        self.begin_arrange(District::Factory, produced);
    }
}
