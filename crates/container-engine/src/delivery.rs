//! Delivery auctions on Container Island.

use crate::config::MAX_PLAYERS;
use crate::game::Game;
use crate::phase::{Bidding, Decision, Phase};
use crate::state::PlayerId;

impl Game {
    /// The ship has just reached the Island: its cargo goes up for auction among the opponents.
    pub(crate) fn start_delivery(&mut self) {
        let deliverer = self.state.active_player;
        let mut contenders = [false; MAX_PLAYERS];
        for seat in (0..self.state.num_players).filter(|&seat| seat != deliverer) {
            contenders[seat] = true;
        }
        let bidder = self
            .next_bidder(deliverer, deliverer, &contenders)
            .expect("a delivery auction has opponents");
        self.phase = Phase::Bid(Bidding::opening(deliverer, bidder, contenders));
    }

    /// The next contender clockwise after `after`, up to but not including the deliverer.
    fn next_bidder(
        &self,
        deliverer: PlayerId,
        after: PlayerId,
        contenders: &[bool; MAX_PLAYERS],
    ) -> Option<PlayerId> {
        let n = self.state.num_players;
        let mut seat = (after + 1) % n;
        while seat != deliverer {
            if contenders[seat] {
                return Some(seat);
            }
            seat = (seat + 1) % n;
        }
        None
    }

    pub(crate) fn can_bid(&self, bidding: &Bidding, amount: u32) -> bool {
        amount <= self.state.players[bidding.bidder].cash
    }

    /// Puts cash on the table, for good. Once every contender has bid, the bids are revealed together.
    pub(crate) fn bid(&mut self, amount: u32) {
        let Phase::Bid(mut bidding) = self.phase else {
            unreachable!("bid outside of a delivery auction");
        };
        let seat = bidding.bidder;
        self.state.players[seat].cash -= amount;
        bidding.bids[seat] += amount;
        match self.next_bidder(bidding.deliverer, bidding.bidder, &bidding.contenders) {
            Some(next) => {
                bidding.bidder = next;
                self.phase = Phase::Bid(bidding);
            }
            None => self.reveal(bidding),
        }
    }

    /// What must hold in every delivery auction, whatever the bidders did.
    pub(crate) fn check_bidding(&self, bidding: &Bidding) -> Result<(), String> {
        if !bidding.contenders[bidding.bidder] {
            return Err(format!("the bidder {} is not a contender", bidding.bidder));
        }
        for seat in 0..MAX_PLAYERS {
            let on_table = bidding.bids[seat];
            if !bidding.contenders[seat] && on_table != 0 {
                return Err(format!(
                    "player {seat} is not bidding but has ${on_table} on the table"
                ));
            }
            if bidding.contenders[seat] && on_table < bidding.base {
                return Err(format!(
                    "player {seat} has ${on_table} on the table, less than the ${} the round began with",
                    bidding.base
                ));
            }
        }
        Ok(())
    }

    /// Everyone but the highest bidders takes their cash back. A tie goes to the one runoff;
    /// after that, or without a tie, the deliverer decides.
    fn reveal(&mut self, bidding: Bidding) {
        let top = (0..self.state.num_players)
            .filter(|&seat| bidding.contenders[seat])
            .map(|seat| bidding.bids[seat])
            .max()
            .expect("a delivery auction has bidders");
        let mut tied = [false; MAX_PLAYERS];
        for seat in (0..self.state.num_players).filter(|&seat| bidding.contenders[seat]) {
            if bidding.bids[seat] == top {
                tied[seat] = true;
            } else {
                self.state.players[seat].cash += bidding.bids[seat];
            }
        }

        let tied_count = tied.iter().filter(|&&is_tied| is_tied).count();
        self.phase = if tied_count > 1 && !bidding.runoff {
            let bidder = self
                .next_bidder(bidding.deliverer, bidding.deliverer, &tied)
                .expect("a tie has bidders");
            Phase::Bid(Bidding::runoff(bidding.deliverer, bidder, tied, top))
        } else {
            Phase::Decide(Decision {
                deliverer: bidding.deliverer,
                top,
                tied,
            })
        };
    }

    /// The deliverer sells the cargo to `winner`: they get the bid and an equal subsidy from the supply.
    pub(crate) fn accept(&mut self, winner: PlayerId) {
        let Phase::Decide(decision) = self.phase else {
            unreachable!("accept outside of a delivery decision");
        };
        self.return_tied_bids(&decision, Some(winner));
        let deliverer = &mut self.state.players[decision.deliverer];
        deliverer.cash += 2 * decision.top;
        self.state.cash_minted += decision.top;
        self.deliver_cargo(decision.deliverer, winner);
    }

    /// The deliverer pays the winning bid to the Bank and keeps the cargo; every bid is returned.
    pub(crate) fn buy_out(&mut self) {
        let Phase::Decide(decision) = self.phase else {
            unreachable!("buy out outside of a delivery decision");
        };
        self.return_tied_bids(&decision, None);
        self.pay_bank_cash(decision.deliverer, decision.top);
        self.deliver_cargo(decision.deliverer, decision.deliverer);
    }

    /// Whoever holds the highest bid takes it back, except `paying`, whose bid is spent.
    fn return_tied_bids(&mut self, decision: &Decision, paying: Option<PlayerId>) {
        for seat in 0..self.state.num_players {
            if decision.tied[seat] && Some(seat) != paying {
                self.state.players[seat].cash += decision.top;
            }
        }
    }

    /// The cargo goes to a scoring area, and the deliverer's turn is over.
    fn deliver_cargo(&mut self, deliverer: PlayerId, to: PlayerId) {
        let cargo = std::mem::take(&mut self.state.players[deliverer].ship);
        self.state.players[to].scoring_area.add_all(&cargo);
        self.end_turn();
    }
}
