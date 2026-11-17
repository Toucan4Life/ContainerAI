//! A compact memory of how each opponent has behaved in this game, built only from what every seat saw:
//! who bought from whom and at what price, and where each player laid its containers out.
//!
//! A net trained with memory reads [`MEM_DIM`] extra inputs, [`PER_OPPONENT`] for each of the other
//! seats in table order from the viewer's left. Nothing hidden is used: no cash, no bids, no scoring card.

use container_engine::action::Action;
use container_engine::config::MAX_PLAYERS;
use container_engine::phase::Phase;

/// Numbers written per opponent.
pub const PER_OPPONENT: usize = 7;
/// Memory inputs of a state: one block per opponent slot.
pub const MEM_DIM: usize = (MAX_PLAYERS - 1) * PER_OPPONENT;

#[derive(Clone, Debug, Default)]
pub struct Memory {
    bought: [u32; MAX_PLAYERS],
    spent: [u32; MAX_PLAYERS],
    sold: [u32; MAX_PLAYERS],
    earned: [u32; MAX_PLAYERS],
    placed: [u32; MAX_PLAYERS],
    placed_fair: [u32; MAX_PLAYERS],
    /// `from[buyer][seller]`: containers bought.
    from: [[u32; MAX_PLAYERS]; MAX_PLAYERS],
}

impl Memory {
    /// Notes `action`, taken by `mover` in `phase`. Calls for actions that tell nothing are no-ops.
    pub fn record(&mut self, mover: usize, phase: &Phase, action: Action) {
        match (phase, action) {
            (Phase::Buy(b), Action::Buy { lot, .. }) => {
                let price = b.district.lot_price(lot);
                self.bought[mover] += 1;
                self.spent[mover] += price;
                self.sold[b.seller] += 1;
                self.earned[b.seller] += price;
                self.from[mover][b.seller] += 1;
            }
            (Phase::Arrange(a), Action::Place(lot)) => {
                self.placed[a.owner] += 1;
                self.placed_fair[a.owner] += u32::from(a.district.lot_price(lot) <= 3);
            }
            _ => {}
        }
    }

    /// Writes the [`MEM_DIM`] inputs for `viewer` at a table of `players`. Per opponent: mean price
    /// paid, containers bought, containers sold, mean price received, share of its containers laid out
    /// at $3 or less, containers it bought from the viewer, containers the viewer bought from it.
    /// Averages are 0 until there is something to average.
    pub fn encode(&self, viewer: usize, players: usize, out: &mut [f32]) {
        let mean = |sum: u32, n: u32| if n > 0 { sum as f32 / n as f32 / 6.0 } else { 0.0 };
        for rel in 1..MAX_PLAYERS {
            let o = &mut out[(rel - 1) * PER_OPPONENT..rel * PER_OPPONENT];
            o.fill(0.0);
            if rel >= players {
                continue;
            }
            let s = (viewer + rel) % players;
            o[0] = mean(self.spent[s], self.bought[s]);
            o[1] = (self.bought[s] as f32 / 10.0).min(2.0);
            o[2] = (self.sold[s] as f32 / 10.0).min(2.0);
            o[3] = mean(self.earned[s], self.sold[s]);
            o[4] = if self.placed[s] > 0 { self.placed_fair[s] as f32 / self.placed[s] as f32 } else { 0.0 };
            o[5] = (self.from[s][viewer] as f32 / 5.0).min(2.0);
            o[6] = (self.from[viewer][s] as f32 / 5.0).min(2.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opponents_are_read_relative_to_the_viewer() {
        let mut m = Memory::default();
        m.bought[2] = 4;
        m.spent[2] = 8;
        m.from[2][1] = 5;
        let mut out = [9.0; MEM_DIM];
        // Seat 2 sits two places to the left of seat 0, and one place to the left of seat 1.
        m.encode(0, 3, &mut out);
        assert!((out[PER_OPPONENT] - 2.0 / 6.0).abs() < 1e-6);
        assert_eq!(out[0], 0.0, "seat 1 has done nothing");
        m.encode(1, 3, &mut out);
        assert!((out[0] - 2.0 / 6.0).abs() < 1e-6);
        assert_eq!(out[5], 1.0, "seat 2 bought five from the viewer");
        assert!(out[2 * PER_OPPONENT..].iter().all(|&x| x == 0.0), "empty seats stay zero");
    }
}
