//! Paying the Off-Shore Bank.

use crate::color::{Color, ColorCounts};
use crate::game::Game;
use crate::state::{AuctionKind, BANK_LOTS, BankState, PlayerId};

impl BankState {
    /// How many lots of this kind can be paid: all but the one with an auction token on it.
    fn open_lot_count(&self, kind: AuctionKind) -> usize {
        BANK_LOTS - usize::from(self.auctions[kind].is_some())
    }

    /// The `n`th lot of this kind that can be paid, counting from lot I.
    fn open_lot(&self, kind: AuctionKind, n: usize) -> usize {
        match self.auctions[kind] {
            Some(auction) if n >= auction.lot => n + 1,
            _ => n,
        }
    }

    /// Deals a cash payment out over the cash lots, $1 at a time from lot I going right and wrapping
    /// around. Every payment starts again at lot I, and a lot with an auction token is skipped.
    pub fn receive_cash(&mut self, amount: u32) {
        let open = self.open_lot_count(AuctionKind::CashLot) as u32;
        for n in 0..open {
            let lot = self.open_lot(AuctionKind::CashLot, n as usize);
            self.cash_lots[lot] += amount / open + u32::from(n < amount % open);
        }
    }

    /// The container lot that the `n`th container of a payment goes to. Every payment starts at lot I,
    /// goes right and wraps around, skipping a lot with an auction token.
    pub fn container_lot_for(&self, n: usize) -> usize {
        self.open_lot(
            AuctionKind::ContainerLot,
            n % self.open_lot_count(AuctionKind::ContainerLot),
        )
    }

    /// Deals a container payment out over the container lots, one at a time and color by color,
    /// starting at lot I and skipping a lot with an auction token.
    pub fn receive_containers(&mut self, payment: &ColorCounts) {
        let mut dealt = 0;
        for color in Color::ALL {
            for _ in 0..payment[color] {
                let lot = self.container_lot_for(dealt);
                self.container_lots[lot][color] += 1;
                dealt += 1;
            }
        }
    }
}

impl Game {
    /// A player pays the Bank out of their hand.
    pub(crate) fn pay_bank_cash(&mut self, payer: PlayerId, amount: u32) {
        self.state.players[payer].cash -= amount;
        self.state.bank.receive_cash(amount);
    }
}
