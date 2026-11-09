//! Rearranging the containers of a district, one container at a time.

use crate::action::District;
use crate::color::ColorCounts;
use crate::game::Game;
use crate::phase::{Arrange, Phase};
use crate::state::{ContainerBid, PlayerId};

impl Game {
    /// Picks up the whole district plus `extra` new containers, to be laid out again one by one.
    pub(crate) fn begin_arrange(&mut self, district: District, extra: ColorCounts) {
        let owner = self.state.active_player;
        let player = &mut self.state.players[owner];
        let mut hand = extra;
        for lot in player.lots_mut(district) {
            hand.add_all(lot);
            *lot = ColorCounts::default();
        }
        if hand.is_empty() {
            self.finish_action();
        } else {
            self.phase = Phase::Arrange(Arrange {
                owner,
                district,
                hand,
                min_lot: 0,
                later: ColorCounts::default(),
            });
        }
    }

    /// A beaten container bid goes back to the districts it came from, factory containers first. Only
    /// these containers are laid out again, and their owner decides where each goes.
    pub(crate) fn begin_return(&mut self, owner: PlayerId, bid: ContainerBid) {
        self.phase = if bid.factory.is_empty() {
            Phase::Arrange(Arrange {
                owner,
                district: District::Harbor,
                hand: bid.harbor,
                min_lot: 0,
                later: ColorCounts::default(),
            })
        } else {
            Phase::Arrange(Arrange {
                owner,
                district: District::Factory,
                hand: bid.factory,
                min_lot: 0,
                later: bid.harbor,
            })
        };
    }

    pub(crate) fn place(&mut self, lot: usize) {
        let Phase::Arrange(mut arrange) = self.phase else {
            unreachable!("place outside of an arrangement");
        };
        let color = arrange
            .hand
            .first_color()
            .expect("the hand is not empty while arranging");
        arrange.hand[color] -= 1;
        self.state.players[arrange.owner].lots_mut(arrange.district)[lot][color] += 1;
        arrange.min_lot = if arrange.hand[color] > 0 {
            lot as u8
        } else {
            0
        };

        if !arrange.hand.is_empty() {
            self.phase = Phase::Arrange(arrange);
        } else if !arrange.later.is_empty() {
            self.phase = Phase::Arrange(Arrange {
                owner: arrange.owner,
                district: District::Harbor,
                hand: arrange.later,
                min_lot: 0,
                later: ColorCounts::default(),
            });
        } else {
            self.finish_action();
        }
    }
}
