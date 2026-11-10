//! An opponent with a few fixed priorities and no look-ahead.

use container_engine::action::Action;
use container_engine::observation::{Observation, PhaseView};
use container_engine::phase::Phase;
use container_engine::rng::Rng;
use container_engine::state::ShipLocation;

use crate::Opponent;

pub const ID: &str = "simple";

/// A player this rich pays a loan back.
const RICH: u32 = 25;

/// Takes the legal action that scores highest, and picks at random among equals. It wants to
/// produce, load the ship and deliver, buys what is cheap and bids a quarter of its cash in
/// delivery auctions. It only takes a loan when it has no cash at all, and it never builds, calls the
/// Bank or buys out.
#[derive(Clone, Debug)]
pub struct Heuristic {
    rng: Rng,
}

impl Heuristic {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: Rng::new(seed),
        }
    }
}

impl Opponent for Heuristic {
    fn choose(&mut self, obs: &Observation, legal: &[Action]) -> Action {
        let scores: Vec<u32> = legal.iter().map(|&a| score(obs, a)).collect();
        let best = *scores.iter().max().expect("there is a legal action");
        let top: Vec<Action> = legal
            .iter()
            .zip(&scores)
            .filter(|&(_, &s)| s == best)
            .map(|(&a, _)| a)
            .collect();
        top[self.rng.below(top.len() as u32) as usize]
    }
}

fn score(obs: &Observation, action: Action) -> u32 {
    let me = &obs.players[obs.viewer];
    let cash = me.cash.unwrap_or(0);
    let loaded = me.ship.total() > 0;
    let at_main = obs.phase == PhaseView::Open(Phase::Main);
    match action {
        Action::RequestLoan if at_main && cash == 0 => 110,
        Action::RepayLoan if cash >= RICH => 105,
        Action::Sail(ShipLocation::Island) if loaded => 100,
        Action::Produce => 90,
        Action::Load(_) => 80,
        Action::DoneLoading => 10,
        Action::DoneBuying => 75,
        Action::HarborPurchase => 70,
        Action::FactoryPurchase(_) => 65,
        // The lowest lot is the cheapest.
        Action::Buy { lot, .. } => 70 - lot as u32,
        Action::Sail(ShipLocation::Ocean) if loaded => 55,
        Action::Sail(ShipLocation::Harbor(_) | ShipLocation::Bank) => 40,
        Action::Sail(_) => 20,
        Action::Reprice(_) => 5,
        Action::Place(lot) => 50 - lot as u32,
        Action::ProduceColor(_) | Action::Seize { .. } | Action::Deal(_) => 50,
        Action::Accept(_) => 50,
        // A quarter of the cash, or as near to it as the bids allow.
        Action::Bid(amount) => 1_000 - amount.abs_diff(cash / 4).min(900),
        Action::BidCash(amount) => 100 - amount.min(100),
        Action::DoneBidding => 90,
        Action::BidContainer { .. } | Action::StartAuction { .. } | Action::Outbid(_) => 1,
        Action::BuyOut | Action::RequestLoan | Action::RepayLoan | Action::Build(_) => 0,
    }
}
