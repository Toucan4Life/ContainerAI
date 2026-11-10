//! The end of the game: the supply runs dry, the Bank settles its auctions, and the players are scored.

use crate::action::Action;
use crate::color::{Color, ColorCounts};
use crate::game::Game;
use crate::loans::MAX_LOANS;
use crate::phase::{Phase, Turn};
use crate::scoring::{FinalResults, final_results};
use crate::state::{Auction, AuctionKind, PlayerState};

/// The game ends once the supply has run out of containers of this many colors.
pub const COLORS_TO_END: usize = 2;

impl Game {
    /// Whether the supply would be out of enough colors once these containers were taken from it.
    pub(crate) fn ends_game_after_taking(&self, taking: &ColorCounts) -> bool {
        let empty = Color::ALL
            .into_iter()
            .filter(|&color| self.state.supply.containers[color] <= taking[color])
            .count();
        empty >= COLORS_TO_END
    }

    /// The supply has run out, so the active player finishes their turn and then the game is over.
    pub fn is_last_turn(&self) -> bool {
        self.ends_game_after_taking(&ColorCounts::default())
    }

    /// Nobody can ever act again to any effect: every player is out of cash, owes every loan there is
    /// and has no container, no Bank auction is running, and each seat has only idle moves (sailing
    /// and repricing). The game ends here, as it could not progress.
    pub fn is_deadlocked(&self) -> bool {
        let broke = |p: &PlayerState| {
            let lots = p.factory_lots.iter().chain(&p.harbor_lots);
            p.cash == 0
                && p.loans == MAX_LOANS
                && [&p.holding, &p.ship, &p.scoring_area]
                    .into_iter()
                    .chain(lots)
                    .all(ColorCounts::is_empty)
        };
        if !self.state.players().iter().all(broke) || self.state.bank.auctions.active() > 0 {
            return false;
        }
        (0..self.state.num_players).all(|seat| {
            let mut probe = self.clone();
            probe.state.active_player = seat;
            probe.turn = Turn::new();
            probe.phase = Phase::Main;
            probe
                .legal_actions()
                .iter()
                .all(|a| matches!(a, Action::Sail(_) | Action::Reprice(_)))
        })
    }

    /// The last turn is over. Every Bank auction is won by its highest bidder, and nobody else's
    /// turn begins.
    pub(crate) fn end_game(&mut self) {
        for kind in AuctionKind::ALL {
            let Some(auction) = self.state.bank.auctions[kind] else {
                continue;
            };
            match kind {
                AuctionKind::ContainerLot => self.win_container_lot(auction),
                AuctionKind::CashLot => self.pay_out_cash_lot(auction),
            }
        }
        self.phase = Phase::GameOver;
    }

    /// The winner takes the cash lot and the Bank takes their containers. Where those go is not
    /// scored, so the winner has no say in it.
    fn pay_out_cash_lot(&mut self, auction: Auction) {
        let bid = self.close_cash_lot_auction(&auction);
        self.state.bank.receive_containers(&bid.all());
        self.collect_cash_lot(auction.holder, auction.lot);
    }

    /// The final scores, once the game is over.
    pub fn results(&self) -> Option<FinalResults> {
        self.is_terminal().then(|| final_results(&self.state))
    }
}
