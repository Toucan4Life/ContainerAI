//! The `Game` type: configuration plus mutable state.

use crate::action::{Action, ActionError, Building, District};
use crate::auction::{MAX_CONTAINER_BID, max_active_auctions};
use crate::color::{Color, ColorCounts, NUM_COLORS};
use crate::config::{ConfigError, GameConfig};
use crate::loans::MAX_LOANS;
use crate::phase::{Phase, Turn};
use crate::rng::Rng;
use crate::state::{
    Auction, AuctionKind, BANK_LOTS, BANK_START_CASH, Bid, HARBOR_LOTS, PlayerId, SHIP_CAPACITY,
    STARTING_CASH, ShipLocation, State,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Game {
    pub config: GameConfig,
    pub state: State,
    pub turn: Turn,
    pub phase: Phase,
}

impl Game {
    pub fn new(config: GameConfig) -> Result<Game, ConfigError> {
        config.validate()?;
        let mut rng = Rng::new(config.seed);
        let state = State::setup(&config, &mut rng);
        Ok(Game {
            config,
            state,
            turn: Turn::new(),
            phase: Phase::Main,
        })
    }

    /// The player who must make the next decision.
    pub fn current_player(&self) -> PlayerId {
        match self.phase {
            Phase::Bid(bidding) => bidding.bidder,
            Phase::Decide(decision) => decision.deliverer,
            Phase::Seize(seizing) => self.state.player_on_right(seizing.debtor),
            Phase::Distribute(dealing) => dealing.decider,
            Phase::Arrange(arrange) => arrange.owner,
            _ => self.state.active_player,
        }
    }

    pub fn is_terminal(&self) -> bool {
        self.phase == Phase::GameOver
    }

    /// Every action the current player may take now, in a fixed order.
    pub fn legal_actions(&self) -> Vec<Action> {
        let mut actions = Vec::new();
        match self.phase {
            Phase::Main => {
                actions.push(Action::Produce);
                actions.extend(Color::ALL.map(|c| Action::Build(Building::Factory(c))));
                actions.push(Action::Build(Building::Warehouse));
                actions.extend((0..self.state.num_players).map(Action::FactoryPurchase));
                actions.push(Action::HarborPurchase);
                actions.extend([
                    Action::Sail(ShipLocation::Ocean),
                    Action::Sail(ShipLocation::Island),
                    Action::Sail(ShipLocation::Bank),
                ]);
                actions.extend(
                    (0..self.state.num_players)
                        .map(|owner| Action::Sail(ShipLocation::Harbor(owner))),
                );
                actions.extend([
                    Action::Reprice(District::Factory),
                    Action::Reprice(District::Harbor),
                    Action::RequestLoan,
                    Action::RepayLoan,
                ]);
                for kind in AuctionKind::ALL {
                    actions.extend((0..BANK_LOTS).map(|lot| Action::StartAuction { kind, lot }));
                }
                actions.extend(AuctionKind::ALL.map(Action::Outbid));
            }
            Phase::Produce(_) => actions.extend(Color::ALL.map(Action::ProduceColor)),
            Phase::Buy(buying) => {
                for lot in 0..buying.district.lot_count() {
                    actions.extend(Color::ALL.map(|color| Action::Buy { lot, color }));
                }
                actions.push(Action::DoneBuying);
            }
            Phase::Load(_) => {
                actions.extend(Color::ALL.map(Action::Load));
                actions.push(Action::DoneLoading);
            }
            Phase::Bid(bidding) => {
                let cash = self.state.players[bidding.bidder].cash;
                actions.extend((0..=cash).map(Action::Bid));
                actions.push(Action::RequestLoan);
            }
            Phase::Decide(_) => {
                actions.extend((0..self.state.num_players).map(Action::Accept));
                actions.extend([Action::BuyOut, Action::RequestLoan]);
            }
            Phase::Seize(_) => {
                for lot in 0..HARBOR_LOTS {
                    actions.extend(Color::ALL.map(|color| Action::Seize { lot, color }));
                }
            }
            Phase::Distribute(_) => actions.extend(Color::ALL.map(Action::Deal)),
            Phase::BankBid(bidding) => match bidding.kind {
                AuctionKind::ContainerLot => {
                    let cash = self.state.players[self.state.active_player].cash;
                    actions.extend((bidding.min..=cash).map(Action::BidCash));
                }
                AuctionKind::CashLot => {
                    for district in [District::Factory, District::Harbor] {
                        for lot in 0..district.lot_count() {
                            actions.extend(Color::ALL.map(|color| Action::BidContainer {
                                district,
                                lot,
                                color,
                            }));
                        }
                    }
                    actions.push(Action::DoneBidding);
                }
            },
            Phase::Arrange(a) => {
                actions.extend((usize::from(a.min_lot)..a.district.lot_count()).map(Action::Place));
            }
            Phase::GameOver => {}
        }
        actions.retain(|&a| self.is_legal(a));
        actions
    }

    /// The single source of truth for legality; `legal_actions` filters through it.
    pub fn is_legal(&self, action: Action) -> bool {
        match (self.phase, action) {
            (Phase::Main, Action::Produce) => self.can_produce(),
            (Phase::Produce(choice), Action::ProduceColor(color)) => {
                self.can_produce_color(&choice, color)
            }
            (Phase::Main, Action::Build(building)) => self.can_build(building),
            (Phase::Main, Action::FactoryPurchase(seller)) => {
                self.can_start_buying(seller, District::Factory)
            }
            (Phase::Main, Action::HarborPurchase) => self.can_harbor_purchase(),
            (Phase::Main, Action::Sail(destination)) => self.can_sail(destination),
            (Phase::Buy(buying), Action::Buy { lot, color }) => self.can_buy(&buying, lot, color),
            (Phase::Buy(buying), Action::DoneBuying) => {
                buying.optional || !buying.bought.is_empty()
            }
            (Phase::Load(loading), Action::Load(color)) => self.can_load_color(&loading, color),
            (Phase::Load(loading), Action::DoneLoading) => loading.optional || loading.loaded > 0,
            (Phase::Bid(bidding), Action::Bid(amount)) => self.can_bid(&bidding, amount),
            (Phase::Decide(decision), Action::Accept(winner)) => {
                winner < self.state.num_players && decision.tied[winner]
            }
            (Phase::Decide(decision), Action::BuyOut) => {
                self.state.players[decision.deliverer].cash >= decision.top
            }
            (Phase::Main, Action::Reprice(district)) => {
                district == District::Factory
                    || self.state.players[self.state.active_player].stock(district) > 0
            }
            (Phase::Arrange(a), Action::Place(lot)) => {
                lot >= usize::from(a.min_lot) && lot < a.district.lot_count()
            }
            (_, Action::RequestLoan) => self.can_request_loan(),
            (_, Action::RepayLoan) => self.can_repay_loan(),
            (Phase::Seize(seizing), Action::Seize { lot, color }) => {
                self.can_seize(&seizing, lot, color)
            }
            (Phase::Distribute(dealing), Action::Deal(color)) => dealing.hand[color] > 0,
            (Phase::Main, Action::StartAuction { kind, lot }) => self.can_start_auction(kind, lot),
            (Phase::Main, Action::Outbid(kind)) => self.can_outbid(kind),
            (Phase::BankBid(bidding), Action::BidCash(amount)) => {
                self.can_bid_cash(&bidding, amount)
            }
            (
                Phase::BankBid(bidding),
                Action::BidContainer {
                    district,
                    lot,
                    color,
                },
            ) => self.can_bid_container(&bidding, district, lot, color),
            (Phase::BankBid(bidding), Action::DoneBidding) => bidding.picked.total() >= bidding.min,
            _ => false,
        }
    }

    pub fn apply(&mut self, action: Action) -> Result<(), ActionError> {
        if !self.is_legal(action) {
            return Err(ActionError::Illegal(action));
        }
        match action {
            Action::Produce => self.produce(),
            Action::ProduceColor(color) => self.produce_color(color),
            Action::Build(building) => self.build(building),
            Action::FactoryPurchase(seller) => self.start_buying(seller, District::Factory, false),
            Action::HarborPurchase => self.harbor_purchase(),
            Action::Sail(destination) => self.sail(destination),
            Action::Buy { lot, color } => self.buy(lot, color),
            Action::DoneBuying => self.done_buying(),
            Action::Load(color) => self.load(color),
            Action::DoneLoading => self.finish_action(),
            Action::Bid(amount) => self.bid(amount),
            Action::Accept(winner) => self.accept(winner),
            Action::BuyOut => self.buy_out(),
            Action::Reprice(district) => self.begin_arrange(district, ColorCounts::default()),
            Action::Place(lot) => self.place(lot),
            Action::RequestLoan => self.request_loan(),
            Action::RepayLoan => self.repay_loan(),
            Action::Seize { lot, color } => self.seize(lot, color),
            Action::Deal(color) => self.deal(color),
            Action::StartAuction { kind, lot } => self.start_auction(kind, lot),
            Action::Outbid(kind) => self.outbid(kind),
            Action::BidCash(amount) => self.bid_cash(amount),
            Action::BidContainer {
                district,
                lot,
                color,
            } => self.bid_container(district, lot, color),
            Action::DoneBidding => self.done_bidding(),
        }
        Ok(())
    }

    /// Called when the current action is fully resolved.
    pub(crate) fn finish_action(&mut self) {
        self.turn.actions_left -= 1;
        if self.turn.actions_left == 0 {
            self.end_turn();
        } else {
            self.phase = Phase::Main;
        }
    }

    pub(crate) fn end_turn(&mut self) {
        if self.is_last_turn() || self.is_deadlocked() {
            self.end_game();
            return;
        }
        self.state.active_player = (self.state.active_player + 1) % self.state.num_players;
        self.begin_turn();
    }

    /// Containers per color everywhere, including those held in hand mid-decision.
    pub fn container_totals(&self) -> [u32; NUM_COLORS] {
        let mut totals = self.state.container_totals();
        let in_flight = self.phase.in_flight();
        for c in Color::ALL {
            totals[c.index()] += u32::from(in_flight[c]);
        }
        totals
    }

    /// Cash everywhere: player hands, the Bank's cash lots, bids on the table and cash on bid tiles.
    pub fn cash_total(&self) -> u32 {
        let hands: u32 = self.state.players().iter().map(|p| p.cash).sum();
        hands
            + self.state.bank.cash_lots.iter().sum::<u32>()
            + self.state.bank.auctions.cash_on_tiles()
            + self.phase.in_flight_cash()
    }

    /// Verifies conservation laws and capacity limits; used by tests and playouts.
    pub fn check_invariants(&self) -> Result<(), String> {
        let st = &self.state;
        let containers = u32::from(self.config.containers_per_color());
        for (c, total) in Color::ALL.into_iter().zip(self.container_totals()) {
            if total != containers {
                return Err(format!(
                    "container conservation: {c:?} has {total}, expected {containers}"
                ));
            }
        }

        let factories = u32::from(self.config.factories_per_color());
        for c in Color::ALL {
            let owned = st.players().iter().filter(|p| p.has_factory(c)).count() as u32;
            let total = u32::from(st.supply.factories[c]) + owned;
            if total != factories {
                return Err(format!(
                    "factory conservation: {c:?} has {total}, expected {factories}"
                ));
            }
        }

        let owned: u32 = st.players().iter().map(|p| u32::from(p.warehouses)).sum();
        let warehouses = u32::from(st.supply.warehouses) + owned;
        let expected = u32::from(self.config.total_warehouses());
        if warehouses != expected {
            return Err(format!(
                "warehouse conservation: {warehouses} in play, expected {expected}"
            ));
        }

        let cash = self.cash_total();
        let expected_cash =
            self.config.num_players as u32 * STARTING_CASH + BANK_START_CASH + st.cash_minted
                - st.cash_burned;
        if cash != expected_cash {
            return Err(format!(
                "cash conservation: {cash} in play, expected {expected_cash}"
            ));
        }
        if let Phase::Bid(bidding) = self.phase {
            self.check_bidding(&bidding)?;
        }

        let auctions = st.bank.auctions.active();
        let allowed = max_active_auctions(st.num_players);
        if auctions > allowed {
            return Err(format!(
                "{auctions} Bank auctions are active, at most {allowed} with {} players",
                st.num_players
            ));
        }

        for kind in AuctionKind::ALL {
            let Some(auction) = st.bank.auctions[kind] else {
                continue;
            };
            match (kind, auction.bid) {
                (AuctionKind::ContainerLot, Bid::Cash(_)) => {}
                (AuctionKind::CashLot, Bid::Containers(bid)) => {
                    if bid.total() > MAX_CONTAINER_BID {
                        return Err(format!(
                            "a bid of {} containers, at most {MAX_CONTAINER_BID} fit on the tile",
                            bid.total()
                        ));
                    }
                }
                (kind, bid) => {
                    return Err(format!("the {kind:?} auction holds the wrong bid: {bid:?}"));
                }
            }
        }
        let container_bid = match st.bank.auctions[AuctionKind::CashLot] {
            Some(Auction {
                holder,
                bid: Bid::Containers(bid),
                ..
            }) => Some((holder, bid)),
            _ => None,
        };

        for (seat, p) in st.players().iter().enumerate() {
            let reserves = (u32::from(p.reserve_factory), u32::from(p.reserve_harbor));
            let on_tile = match container_bid {
                Some((holder, bid)) if holder == seat => (bid.factory.total(), bid.harbor.total()),
                _ => (0, 0),
            };
            if reserves != on_tile {
                return Err(format!(
                    "player {seat}: reserve tokens {reserves:?}, but the containers on the tile are {on_tile:?}"
                ));
            }
            let ship = p.ship.total();
            if ship > SHIP_CAPACITY {
                return Err(format!("player {seat}: ship capacity exceeded ({ship})"));
            }
            if p.loans > MAX_LOANS {
                return Err(format!(
                    "player {seat}: {} loans, at most {MAX_LOANS}",
                    p.loans
                ));
            }
            let factory_used = p.factory_stock() + reserves.0;
            if factory_used > p.factory_limit() {
                return Err(format!(
                    "player {seat}: factory district capacity exceeded ({factory_used} > {})",
                    p.factory_limit()
                ));
            }
            let harbor_used = p.harbor_stock() + reserves.1;
            if harbor_used > p.harbor_limit() {
                return Err(format!(
                    "player {seat}: harbor capacity exceeded ({harbor_used} > {})",
                    p.harbor_limit()
                ));
            }
        }
        Ok(())
    }
}
