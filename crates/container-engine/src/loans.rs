//! Bank loans, and the interest paid on them when a turn begins.

use crate::game::Game;
use crate::phase::{Phase, Turn};
use crate::state::PlayerId;

/// Loans a player can owe at once.
pub const MAX_LOANS: u8 = 2;
/// Cash a loan pays out, and costs to repay.
pub const LOAN_AMOUNT: u32 = 10;
/// Loans a player may request, and may repay, in one of their own turns.
pub const LOANS_PER_TURN: u8 = 2;

impl Game {
    /// Whether the player who must decide now may take a loan. Bidders and the deliverer deciding on
    /// their auction may do so as well as the active player between actions.
    pub(crate) fn can_request_loan(&self) -> bool {
        let borrower = self.current_player();
        if self.state.players[borrower].loans >= MAX_LOANS {
            return false;
        }
        match self.phase {
            Phase::Main | Phase::Decide(_) => self.turn.loans_requested < LOANS_PER_TURN,
            Phase::Bid(_) => true,
            _ => false,
        }
    }

    /// Loans are only repaid on your own turn, between actions.
    pub(crate) fn can_repay_loan(&self) -> bool {
        let player = &self.state.players[self.state.active_player];
        matches!(self.phase, Phase::Main)
            && player.loans > 0
            && player.cash >= LOAN_AMOUNT
            && self.turn.loans_repaid < LOANS_PER_TURN
    }

    pub(crate) fn request_loan(&mut self) {
        let borrower = self.current_player();
        self.take_loan(borrower);
        // A bidder is not on their own turn, so the active player's allowance is left alone.
        if borrower == self.state.active_player {
            self.turn.loans_requested += 1;
        }
    }

    fn take_loan(&mut self, player: PlayerId) {
        let borrower = &mut self.state.players[player];
        borrower.loans += 1;
        borrower.cash += LOAN_AMOUNT;
        self.state.cash_minted += LOAN_AMOUNT;
    }

    pub(crate) fn repay_loan(&mut self) {
        let borrower = &mut self.state.players[self.state.active_player];
        borrower.loans -= 1;
        borrower.cash -= LOAN_AMOUNT;
        self.state.cash_burned += LOAN_AMOUNT;
        self.turn.loans_repaid += 1;
    }

    /// The active player's turn begins: the interest is paid before anything else is done.
    pub(crate) fn begin_turn(&mut self) {
        self.turn = Turn::new();
        let player = self.state.active_player;
        match self.pay_interest(player) {
            0 => self.win_auctions(),
            unpaid => self.begin_seizure(player, unpaid),
        }
    }

    /// Pays $1 per loan to the Bank, and returns how many loans' interest could not be paid. A player
    /// who is short and has room for another loan takes it to pay the interest.
    fn pay_interest(&mut self, player: PlayerId) -> u8 {
        let loans = self.state.players[player].loans;
        let due = u32::from(loans);
        if self.state.players[player].cash < due && loans < MAX_LOANS {
            self.take_loan(player);
        }
        let paid = self.state.players[player].cash.min(due);
        self.pay_bank_cash(player, paid);
        (due - paid) as u8
    }
}
