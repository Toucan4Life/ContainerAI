//! The bot of the solitaire variant (`Solo_Variant_v1.1.pdf`), played as an ordinary seat.
//!
//! The variant scripts a bot with a fixed action order (sail, produce, buy from a factory, buy from a
//! harbor, reprice, build) and a d20 for every price and every purchase. This port keeps the order and
//! the tables. Where the variant gives the bots no money, here the bot has real cash, so it bids the
//! rolled value of the shipment, borrows when it has none and pays loans back when rich.
//!
//! The variant does not say what the bots do with the Off-Shore Bank. This port adds one step to the list,
//! after the harbor purchase and before the reprice: a bot that could not sail, produce or buy rolls a d20
//! and calls the Bank on 11+ for the container lot with the most containers (the cheapest lot among
//! equals), bidding by the shipment table ($3 on 1-4, up to $7 on 17-20; the least allowed, if the roll
//! is lower). When an opponent holds the container lot, it outbids them on 16+. It never bids containers
//! for a cash lot. The die is a seeded [`Rng`], so a seed repeats the game.

use container_engine::action::{Action, Building, District};
use container_engine::observation::{Observation, PhaseView};
use container_engine::phase::Phase;
use container_engine::rng::Rng;
use container_engine::state::{AuctionKind, ShipLocation};

use crate::Opponent;

pub const ID: &str = "solitaire";

/// A player this rich pays a loan back.
const RICH: u32 = 25;

#[derive(Clone, Debug)]
pub struct Solitaire {
    rng: Rng,
    /// The most a purchase may cost, set by the one d20 rolled when the purchase action is chosen.
    cap: u32,
}

impl Solitaire {
    pub fn new(seed: u64) -> Self {
        Self { rng: Rng::new(seed), cap: 0 }
    }

    fn d20(&mut self) -> u32 {
        self.rng.below(20) + 1
    }

    /// Price of a container laid out in a factory district.
    fn factory_price(&mut self) -> u32 {
        (self.d20() - 1) / 5 + 1
    }

    /// Price of a container laid out in a harbor district.
    fn harbor_price(&mut self) -> u32 {
        (self.d20() - 1) / 4 + 2
    }

    /// Value of the shipment: one roll per type of container.
    fn shipment_value(&mut self, types: usize) -> u32 {
        (0..types).map(|_| (self.d20() - 1) / 4 + 3).sum()
    }

    /// 11+ calls the Bank on the fullest container lot; an opponent's cash bid is outbid on 16+.
    fn bank_call(&mut self, obs: &Observation, legal: &[Action]) -> Option<Action> {
        let kind = AuctionKind::ContainerLot;
        if legal.contains(&Action::Outbid(kind)) && self.d20() >= 16 {
            return Some(Action::Outbid(kind));
        }
        let lot = legal
            .iter()
            .filter_map(|a| match a {
                Action::StartAuction { kind: AuctionKind::ContainerLot, lot } => Some(*lot),
                _ => None,
            })
            .max_by_key(|&lot| (obs.bank.container_lots[lot].total(), std::cmp::Reverse(lot)))?;
        (self.d20() >= 11).then_some(Action::StartAuction { kind, lot })
    }

    fn factory_cap(&mut self) -> u32 {
        match self.d20() {
            1..=5 => 1,
            6..=10 => 2,
            11..=15 => 3,
            _ => 4,
        }
    }

    fn harbor_cap(&mut self) -> u32 {
        match self.d20() {
            1..=4 => 2,
            5..=8 => 3,
            9..=12 => 4,
            13..=16 => 5,
            _ => 6,
        }
    }
}

/// The cheapest container a district offers that the cap allows and the cash pays, and its mean price.
fn cheapest(lots: &[container_engine::color::ColorCounts], first_price: u32, cap: u32, cash: u32) -> Option<f64> {
    let (mut total, mut count, mut any) = (0, 0, false);
    for (lot, c) in lots.iter().enumerate() {
        let price = lot as u32 + first_price;
        total += price * c.total();
        count += c.total();
        any |= c.total() > 0 && price <= cap && price <= cash;
    }
    (any && count > 0).then(|| f64::from(total) / f64::from(count))
}

impl Opponent for Solitaire {
    fn choose(&mut self, obs: &Observation, legal: &[Action]) -> Action {
        let has = |a: Action| legal.contains(&a);
        let me = &obs.players[obs.viewer];
        let cash = me.cash.unwrap_or(0);
        let first = legal[0];
        match obs.phase {
            PhaseView::Bid(b) => {
                let ship = obs.players[b.deliverer].ship;
                let types = ship.iter_nonzero().count();
                let want = self.shipment_value(types).min(cash);
                return legal
                    .iter()
                    .copied()
                    .filter(|a| matches!(a, Action::Bid(x) if *x <= want))
                    .max_by_key(|a| if let Action::Bid(x) = a { *x } else { 0 })
                    .unwrap_or(first);
            }
            PhaseView::Open(Phase::Decide(d)) => {
                let types = me.ship.iter_nonzero().count().max(1);
                let worth = self.shipment_value(types);
                let accept = legal.iter().copied().find(|a| matches!(a, Action::Accept(_)));
                return match accept {
                    Some(a) if d.top >= worth || !has(Action::BuyOut) => a,
                    _ => Action::BuyOut,
                };
            }
            PhaseView::Open(Phase::Arrange(a)) => {
                let target = match a.district {
                    District::Factory => self.factory_price() as i64 - 1,
                    District::Harbor => self.harbor_price() as i64 - 2,
                };
                return legal
                    .iter()
                    .copied()
                    .filter(|x| matches!(x, Action::Place(_)))
                    .min_by_key(|x| if let Action::Place(l) = x { (*l as i64 - target).abs() } else { 0 })
                    .unwrap_or(first);
            }
            PhaseView::Open(Phase::Buy(b)) => {
                let cap = self.cap;
                let cheap = legal
                    .iter()
                    .copied()
                    .filter_map(|a| match a {
                        Action::Buy { lot, .. } => {
                            let base = if b.district == District::Factory { 1 } else { 2 };
                            (lot as u32 + base <= cap).then_some((lot, a))
                        }
                        _ => None,
                    })
                    .min_by_key(|&(lot, _)| lot);
                return match cheap {
                    Some((_, a)) => a,
                    None if has(Action::DoneBuying) => Action::DoneBuying,
                    None => first,
                };
            }
            PhaseView::Open(Phase::Load(_)) => {
                return legal.iter().copied().find(|a| matches!(a, Action::Load(_))).unwrap_or(Action::DoneLoading);
            }
            PhaseView::Open(Phase::BankBid(_)) => {
                let want = self.shipment_value(1);
                let bids = || legal.iter().filter_map(|a| if let Action::BidCash(x) = a { Some(*x) } else { None });
                return match bids().filter(|&x| x <= want).max().or_else(|| bids().min()) {
                    Some(x) => Action::BidCash(x),
                    None => first,
                };
            }
            PhaseView::Open(Phase::Main) => {}
            PhaseView::Open(_) => return first,
        }

        if cash == 0 && has(Action::RequestLoan) {
            return Action::RequestLoan;
        }
        if cash >= RICH && has(Action::RepayLoan) {
            return Action::RepayLoan;
        }

        // a. Sail: out of a harbor or the Island, or to the Island with a cargo.
        let loaded = me.ship.total() > 0;
        if loaded && has(Action::Sail(ShipLocation::Island)) {
            return Action::Sail(ShipLocation::Island);
        }
        if !matches!(me.ship_location, ShipLocation::Ocean) && has(Action::Sail(ShipLocation::Ocean)) {
            if !matches!(me.ship_location, ShipLocation::Harbor(_)) || !has(Action::HarborPurchase) {
                return Action::Sail(ShipLocation::Ocean);
            }
        }
        // b. Produce, when the factory district lacks a type or is empty.
        let stocked: u32 = me.factory_lots.iter().map(|c| c.total()).sum();
        if has(Action::Produce) && stocked == 0 {
            return Action::Produce;
        }
        // c. Buy from a factory: the seller with the lowest mean price, if the die allows a container.
        let others: Vec<usize> = (0..obs.num_players).filter(|&p| p != obs.viewer).collect();
        let cap = self.factory_cap();
        let seller = others
            .iter()
            .filter(|&&p| has(Action::FactoryPurchase(p)))
            .filter_map(|&p| cheapest(&obs.players[p].factory_lots, 1, cap, cash).map(|m| (m, p)))
            .min_by(|a, b| a.0.total_cmp(&b.0));
        if let Some((_, p)) = seller {
            self.cap = cap;
            return Action::FactoryPurchase(p);
        }
        // d. Buy from a harbor: docked at one, or sail to the cheapest.
        let cap = self.harbor_cap();
        if let ShipLocation::Harbor(p) = me.ship_location {
            if has(Action::HarborPurchase) && cheapest(&obs.players[p].harbor_lots, 2, cap, cash).is_some() {
                self.cap = cap;
                return Action::HarborPurchase;
            }
        }
        let harbor = others
            .iter()
            .filter(|&&p| has(Action::Sail(ShipLocation::Harbor(p))))
            .filter_map(|&p| cheapest(&obs.players[p].harbor_lots, 2, cap, cash).map(|m| (m, p)))
            .min_by(|a, b| a.0.total_cmp(&b.0));
        if let Some((_, p)) = harbor {
            self.cap = cap;
            return Action::Sail(ShipLocation::Harbor(p));
        }
        // The Bank call.
        if let Some(a) = self.bank_call(obs, legal) {
            return a;
        }
        // Produce, even with some stock, rather than idle.
        if has(Action::Produce) && stocked < 2 {
            return Action::Produce;
        }
        // e. Reprice $4 factory or $6 harbor containers.
        if has(Action::Reprice(District::Factory)) && me.factory_lots[3].total() > 0 {
            return Action::Reprice(District::Factory);
        }
        if has(Action::Reprice(District::Harbor)) && me.harbor_lots[4].total() > 0 {
            return Action::Reprice(District::Harbor);
        }
        // f. Build, if an opponent has more buildings than the bot.
        let size = |p: usize| {
            let v = &obs.players[p];
            v.factories.iter().filter(|&&f| f).count() as u8 + v.warehouses
        };
        let mine = size(obs.viewer);
        if others.iter().any(|&p| size(p) > mine) {
            let wanted = if me.warehouses < me.factories.iter().filter(|&&f| f).count() as u8 {
                Building::Warehouse
            } else {
                Building::Factory(container_engine::color::Color::ALL[0])
            };
            if has(Action::Build(wanted)) {
                return Action::Build(wanted);
            }
            if let Some(a) = legal.iter().copied().find(|a| matches!(a, Action::Build(_))) {
                return a;
            }
        }
        // Nothing in the list applies: pass with the least harmful action. Sailing to the Bank is the last
        // resort, and a Bank call never.
        let rank = |a: &Action| match a {
            Action::Produce => 0,
            Action::Reprice(_) => 1,
            Action::Build(_) => 2,
            Action::Sail(ShipLocation::Harbor(_)) => 3,
            Action::Sail(ShipLocation::Island) | Action::Sail(ShipLocation::Ocean) => 4,
            Action::Sail(_) => 5,
            Action::StartAuction { .. } | Action::Outbid(_) | Action::RequestLoan => 9,
            _ => 6,
        };
        legal.iter().copied().min_by_key(rank).unwrap_or(first)
    }
}
