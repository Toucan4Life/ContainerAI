//! Off-Shore Bank auctions: calling the Bank, bidding, and collecting a won lot.

use crate::action::District;
use crate::color::Color;
use crate::game::Game;
use crate::phase::{BankBidding, Distributing, Phase};
use crate::state::{Auction, AuctionKind, BANK_LOTS, Bid, ContainerBid, PlayerId};

/// The least a Bank auction may be opened with: $1 or one container.
pub const OPENING_BID: u32 = 1;

/// The most containers a bid can hold, as there are only 10 reserve tokens.
pub const MAX_CONTAINER_BID: u32 = 10;

/// How many auctions the Bank can hold at once: one, or with five players one of each kind.
pub const fn max_active_auctions(players: usize) -> usize {
    if players >= 5 { 2 } else { 1 }
}

impl Game {
    /// The Bank is called once per turn, and not in a turn in which an auction was won. Nor can it be
    /// called once the supply has run out, as the game ends with this turn.
    fn can_call_bank(&self) -> bool {
        !self.turn.bank_called && !self.turn.won_auction && !self.is_last_turn()
    }

    /// An auction opens on a lot that holds something, as long as the Bank has room for it and the
    /// opening bid can be paid: $1 for a container lot, a container of the factory or harbor district
    /// for a cash lot.
    pub(crate) fn can_start_auction(&self, kind: AuctionKind, lot: usize) -> bool {
        let bank = &self.state.bank;
        let me = &self.state.players[self.state.active_player];
        self.can_call_bank()
            && lot < BANK_LOTS
            && bank.auctions[kind].is_none()
            && bank.auctions.active() < max_active_auctions(self.state.num_players)
            && match kind {
                AuctionKind::ContainerLot => {
                    !bank.container_lots[lot].is_empty() && me.cash >= OPENING_BID
                }
                AuctionKind::CashLot => bank.cash_lots[lot] > 0 && me.bid_stock() >= OPENING_BID,
            }
    }

    /// Only an opponent's bid can be beaten, and only by paying more than it: more dollars, or more
    /// containers than there are on the tile, up to the 10 that reserve tokens allow.
    pub(crate) fn can_outbid(&self, kind: AuctionKind) -> bool {
        let Some(auction) = self.state.bank.auctions[kind] else {
            return false;
        };
        let me = self.state.active_player;
        let player = &self.state.players[me];
        let to_beat = auction.bid.value();
        self.can_call_bank()
            && auction.holder != me
            && match kind {
                AuctionKind::ContainerLot => player.cash > to_beat,
                AuctionKind::CashLot => to_beat < MAX_CONTAINER_BID && player.bid_stock() > to_beat,
            }
    }

    pub(crate) fn can_bid_cash(&self, bidding: &BankBidding, amount: u32) -> bool {
        bidding.kind == AuctionKind::ContainerLot
            && amount >= bidding.min
            && amount <= self.state.players[self.state.active_player].cash
    }

    /// A container can be picked if it comes after the last one picked, there is a reserve token left
    /// for it, and the bid can still reach its minimum with the containers that come after it.
    pub(crate) fn can_bid_container(
        &self,
        bidding: &BankBidding,
        district: District,
        lot: usize,
        color: Color,
    ) -> bool {
        if bidding.kind != AuctionKind::CashLot || lot >= district.lot_count() {
            return false;
        }
        let player = &self.state.players[self.state.active_player];
        let key = BankBidding::order_key(district, lot, color);
        let picked = bidding.picked.total();
        player.lots(district)[lot][color] > 0
            && key >= bidding.next
            && picked < MAX_CONTAINER_BID
            && picked + self.bid_stock_from(key) >= bidding.min
    }

    /// The active player's containers that are at or after this position in the picking order.
    fn bid_stock_from(&self, key: u8) -> u32 {
        let player = &self.state.players[self.state.active_player];
        let mut stock = 0;
        for district in [District::Factory, District::Harbor] {
            for (lot, counts) in player.lots(district).iter().enumerate() {
                for color in Color::ALL {
                    if BankBidding::order_key(district, lot, color) >= key {
                        stock += u32::from(counts[color]);
                    }
                }
            }
        }
        stock
    }

    pub(crate) fn start_auction(&mut self, kind: AuctionKind, lot: usize) {
        self.phase = Phase::BankBid(BankBidding::new(kind, lot, OPENING_BID));
    }

    pub(crate) fn outbid(&mut self, kind: AuctionKind) {
        let auction = self.state.bank.auctions[kind].expect("there is a bid to beat");
        self.phase = Phase::BankBid(BankBidding::new(kind, auction.lot, auction.bid.value() + 1));
    }

    /// The cash goes on the tile. A player who is outbid gets theirs back.
    pub(crate) fn bid_cash(&mut self, amount: u32) {
        let Phase::BankBid(bidding) = self.phase else {
            unreachable!("a cash bid outside of a Bank bid");
        };
        let bidder = self.state.active_player;
        if let Some(beaten) = self.state.bank.auctions[bidding.kind] {
            self.state.players[beaten.holder].cash += beaten.bid.value();
        }
        self.state.players[bidder].cash -= amount;
        self.state.bank.auctions[bidding.kind] = Some(Auction {
            lot: bidding.lot,
            holder: bidder,
            bid: Bid::Cash(amount),
        });
        self.turn.bank_called = true;
        self.finish_action();
    }

    /// The container leaves its lot and waits in hand until the bid is done.
    pub(crate) fn bid_container(&mut self, district: District, lot: usize, color: Color) {
        let Phase::BankBid(mut bidding) = self.phase else {
            unreachable!("a container bid outside of a Bank bid");
        };
        self.state.players[self.state.active_player].lots_mut(district)[lot][color] -= 1;
        match district {
            District::Factory => bidding.picked.factory[color] += 1,
            District::Harbor => bidding.picked.harbor[color] += 1,
        }
        bidding.next = BankBidding::order_key(district, lot, color);
        self.phase = Phase::BankBid(bidding);
    }

    /// The picked containers go on the tile, and a reserve token takes the place of each in its
    /// district. A player who is outbid gets their containers back, to price again.
    pub(crate) fn done_bidding(&mut self) {
        let Phase::BankBid(bidding) = self.phase else {
            unreachable!("done bidding outside of a Bank bid");
        };
        let bidder = self.state.active_player;
        let beaten = self.state.bank.auctions[bidding.kind];
        self.state.bank.auctions[bidding.kind] = Some(Auction {
            lot: bidding.lot,
            holder: bidder,
            bid: Bid::Containers(bidding.picked),
        });
        let player = &mut self.state.players[bidder];
        player.reserve_factory = bidding.picked.factory.total() as u8;
        player.reserve_harbor = bidding.picked.harbor.total() as u8;
        self.turn.bank_called = true;
        match beaten {
            Some(Auction {
                holder,
                bid: Bid::Containers(bid),
                ..
            }) => {
                self.state.players[holder].reserve_factory = 0;
                self.state.players[holder].reserve_harbor = 0;
                self.begin_return(holder, bid);
            }
            _ => self.finish_action(),
        }
    }

    /// The interest is settled, so the active player now wins every auction they hold the highest bid
    /// in. Then they may act.
    pub(crate) fn win_auctions(&mut self) {
        self.phase = Phase::Main;
        if let Some(auction) = self.held_auction(AuctionKind::ContainerLot) {
            self.turn.won_auction = true;
            self.win_container_lot(auction);
        }
        if let Some(auction) = self.held_auction(AuctionKind::CashLot) {
            self.turn.won_auction = true;
            self.win_cash_lot(auction);
        }
    }

    /// The active auction of this kind, if the active player holds the highest bid in it.
    fn held_auction(&self, kind: AuctionKind) -> Option<Auction> {
        self.state.bank.auctions[kind].filter(|auction| auction.holder == self.state.active_player)
    }

    /// Pays the bid to the Bank, takes the lot into the holding area, and returns the token and tile.
    pub(crate) fn win_container_lot(&mut self, auction: Auction) {
        self.state.bank.receive_cash(auction.bid.value());
        let lot = std::mem::take(&mut self.state.bank.container_lots[auction.lot]);
        self.state.players[auction.holder].holding.add_all(&lot);
        self.state.bank.auctions[AuctionKind::ContainerLot] = None;
    }

    /// Pays the bid to the Bank, which deals the containers out over its container lots as the winner
    /// decides. Once that is done, the winner takes the cash lot and the token, tile and reserve
    /// tokens are returned.
    fn win_cash_lot(&mut self, auction: Auction) {
        let bid = self.close_cash_lot_auction(&auction);
        self.phase = Phase::Distribute(Distributing {
            decider: auction.holder,
            hand: bid.all(),
            dealt: 0,
            won_lot: Some(auction.lot),
        });
        self.advance_dealing();
    }

    /// The token, the tile and the reserve tokens of a cash lot auction are returned, which leaves
    /// the containers of the winning bid to be paid to the Bank.
    pub(crate) fn close_cash_lot_auction(&mut self, auction: &Auction) -> ContainerBid {
        let Bid::Containers(bid) = auction.bid else {
            unreachable!("a cash lot is won with containers");
        };
        let winner = &mut self.state.players[auction.holder];
        winner.reserve_factory = 0;
        winner.reserve_harbor = 0;
        self.state.bank.auctions[AuctionKind::CashLot] = None;
        bid
    }

    /// The winner of a cash lot takes all the cash on it.
    pub(crate) fn collect_cash_lot(&mut self, winner: PlayerId, lot: usize) {
        let collected = std::mem::take(&mut self.state.bank.cash_lots[lot]);
        self.state.players[winner].cash += collected;
    }
}
