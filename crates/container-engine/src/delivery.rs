//! Delivery auctions on Container Island.

use crate::cards::{BLUFF_CARDS, can_show};
use crate::config::MAX_PLAYERS;
use crate::game::Game;
use crate::phase::{BidStep, Bidding, Decision, Phase, Stack};
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
        let seat = bidding.bidder;
        let cash = self.state.players[seat].cash;
        match bidding.step {
            BidStep::Cash => amount <= cash,
            // A revision may spend the cash of the stack it replaces.
            BidStep::Review => {
                bidding.revisions[seat] < self.config.max_bid_revisions
                    && amount <= cash + bidding.stack_amount(seat)
            }
            BidStep::Cards | BidStep::Bluffs => false,
        }
    }

    pub(crate) fn can_bid_cards(&self, bidding: &Bidding, cards: u32) -> bool {
        bidding.step == BidStep::Cards && can_show(bidding.stack_amount(bidding.bidder), cards)
    }

    /// Puts cash on the table. Once every contender has bid, the bids are revealed. When bids may be
    /// revised, the cash is first laid out as cards, before the next contender bids; and a bidder looking
    /// the stacks over may take theirs back and bid again, which makes everyone look once more.
    pub(crate) fn bid(&mut self, amount: u32) {
        let Phase::Bid(mut bidding) = self.phase else {
            unreachable!("bid outside of a delivery auction");
        };
        let seat = bidding.bidder;
        if bidding.step == BidStep::Review {
            self.state.players[seat].cash += bidding.stack_amount(seat);
            bidding.bids[seat] = bidding.base;
            bidding.stacks[seat] = Stack::default();
            bidding.revisions[seat] += 1;
            bidding.ready = [false; MAX_PLAYERS];
        }
        self.state.players[seat].cash -= amount;
        bidding.bids[seat] += amount;
        if self.config.max_bid_revisions > 0 {
            bidding.step = BidStep::Cards;
            self.phase = Phase::Bid(bidding);
            return;
        }
        match self.next_bidder(bidding.deliverer, bidding.bidder, &bidding.contenders) {
            Some(next) => {
                bidding.bidder = next;
                self.phase = Phase::Bid(bidding);
            }
            None => self.reveal(bidding),
        }
    }

    /// The cash just put down lies on the table as this many cash cards.
    pub(crate) fn bid_cards(&mut self, cards: u32) {
        let Phase::Bid(mut bidding) = self.phase else {
            unreachable!("cards outside of a delivery auction");
        };
        bidding.stacks[bidding.bidder].cards = cards;
        bidding.step = BidStep::Bluffs;
        self.phase = Phase::Bid(bidding);
    }

    /// Completes the bidder's stack. The next contender lays out theirs; once the last stack is down,
    /// the number of cards in each is public and the bidders look the stacks over. A bidder who has
    /// just laid out a revised stack has nothing more to say about it.
    pub(crate) fn bid_bluffs(&mut self, bluffs: u8) {
        let Phase::Bid(mut bidding) = self.phase else {
            unreachable!("bluff cards outside of a delivery auction");
        };
        bidding.stacks[bidding.bidder].bluffs = bluffs;
        if bidding.stacks_down {
            bidding.ready[bidding.bidder] = true;
            self.review(bidding);
            return;
        }
        match self.next_bidder(bidding.deliverer, bidding.bidder, &bidding.contenders) {
            Some(next) => {
                bidding.bidder = next;
                bidding.step = BidStep::Cash;
                self.phase = Phase::Bid(bidding);
            }
            None => {
                bidding.stacks_down = true;
                self.review(bidding);
            }
        }
    }

    /// The bidder leaves their stack as it is.
    pub(crate) fn ready_to_reveal(&mut self) {
        let Phase::Bid(mut bidding) = self.phase else {
            unreachable!("being content outside of a delivery auction");
        };
        bidding.ready[bidding.bidder] = true;
        self.review(bidding);
    }

    /// Hands the stacks to the next bidder who may still want to change theirs. When there is none,
    /// the stacks are revealed.
    fn review(&mut self, mut bidding: Bidding) {
        match self.next_reviewer(&bidding) {
            Some(seat) => {
                bidding.bidder = seat;
                bidding.step = BidStep::Review;
                self.phase = Phase::Bid(bidding);
            }
            None => self.reveal(bidding),
        }
    }

    /// The next contender clockwise after the bidder who is not content and could still revise. The
    /// bidder is skipped: they are content, or they have just laid out the last stack and the first
    /// bidder looks first.
    fn next_reviewer(&self, bidding: &Bidding) -> Option<PlayerId> {
        let n = self.state.num_players;
        (1..n)
            .map(|offset| (bidding.bidder + offset) % n)
            .find(|&seat| {
                bidding.contenders[seat]
                    && !bidding.ready[seat]
                    && bidding.revisions[seat] < self.config.max_bid_revisions
            })
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
            if !bidding.contenders[seat] && bidding.stacks[seat] != Stack::default() {
                return Err(format!(
                    "player {seat} is not bidding but has cards laid out"
                ));
            }
            if !bidding.contenders[seat] && (bidding.ready[seat] || bidding.revisions[seat] != 0) {
                return Err(format!(
                    "player {seat} is not bidding but is content or has revised"
                ));
            }
            if bidding.contenders[seat] && on_table < bidding.base {
                return Err(format!(
                    "player {seat} has ${on_table} on the table, less than the ${} the round began with",
                    bidding.base
                ));
            }
        }

        if self.config.max_bid_revisions == 0 {
            let plain = bidding.step == BidStep::Cash
                && !bidding.stacks_down
                && bidding.stacks == [Stack::default(); MAX_PLAYERS]
                && bidding.ready == [false; MAX_PLAYERS]
                && bidding.revisions == [0; MAX_PLAYERS];
            return if plain {
                Ok(())
            } else {
                Err("bids cannot be revised, yet cards are laid out or bidders are content or have revised"
                    .to_string())
            };
        }

        let fits = match bidding.step {
            BidStep::Cash => !bidding.stacks_down,
            BidStep::Cards | BidStep::Bluffs => true,
            BidStep::Review => bidding.stacks_down,
        };
        if !fits {
            return Err(format!(
                "the step {:?} does not fit stacks that are {}",
                bidding.step,
                if bidding.stacks_down {
                    "all down"
                } else {
                    "still to come"
                }
            ));
        }
        let cap = self.config.max_bid_revisions;
        for seat in (0..self.state.num_players).filter(|&seat| bidding.contenders[seat]) {
            let stack = bidding.stacks[seat];
            let amount = bidding.stack_amount(seat);
            let laying_out = seat == bidding.bidder && bidding.step == BidStep::Cards;
            if !laying_out && !can_show(amount, stack.cards) {
                return Err(format!(
                    "player {seat}'s {} cash cards cannot be laid out for ${amount}",
                    stack.cards
                ));
            }
            if stack.bluffs > BLUFF_CARDS {
                return Err(format!(
                    "player {seat} has {} bluff cards, at most {BLUFF_CARDS}",
                    stack.bluffs
                ));
            }
            let revisions = bidding.revisions[seat];
            if revisions > cap {
                return Err(format!(
                    "player {seat} has revised {revisions} times, at most {cap}"
                ));
            }
            if revisions != 0 && !bidding.stacks_down {
                return Err(format!(
                    "player {seat} has revised before every stack was down"
                ));
            }
            if bidding.ready[seat] && bidding.step != BidStep::Review {
                return Err(format!(
                    "player {seat} is content although the stacks are not being looked over"
                ));
            }
        }
        let reviewer = bidding.bidder;
        if bidding.step == BidStep::Review
            && (bidding.ready[reviewer] || bidding.revisions[reviewer] >= cap)
        {
            return Err(format!(
                "the reviewer {reviewer} is content or cannot revise"
            ));
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
