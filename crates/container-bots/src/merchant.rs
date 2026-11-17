//! A tunable cousin of the simple opponent: the same kind of priority list, but every priority and
//! threshold is a parameter, tuned by self-play in the arena (`tune` in `container-arena`).

use container_engine::action::{Action, Building, District};
use container_engine::observation::{Observation, PhaseView};
use container_engine::phase::Phase;
use container_engine::rng::Rng;
use container_engine::state::ShipLocation;

use crate::Opponent;

pub const ID: &str = "merchant";

/// One tunable number: name, default and the range the tuner may move it in.
pub struct Param {
    pub name: &'static str,
    pub default: f64,
    pub lo: f64,
    pub hi: f64,
}

const fn p(name: &'static str, default: f64, lo: f64, hi: f64) -> Param {
    Param {
        name,
        default,
        lo,
        hi,
    }
}

pub const LOAN_BROKE: usize = 0;
pub const LOAN_LOW: usize = 1;
pub const REPAY: usize = 2;
pub const SAIL_ISLAND: usize = 3;
pub const PRODUCE: usize = 4;
pub const LOAD: usize = 5;
pub const DONE_LOADING: usize = 6;
pub const DONE_BUYING: usize = 7;
pub const HARBOR_BUY: usize = 8;
pub const FACTORY_BUY: usize = 9;
pub const BUY: usize = 10;
pub const SAIL_OCEAN: usize = 11;
pub const SAIL_HARBOR: usize = 12;
pub const SAIL_BANK: usize = 13;
pub const REPRICE: usize = 14;
pub const PLACE: usize = 15;
pub const ACCEPT: usize = 16;
pub const BUY_OUT: usize = 17;
pub const DONE_BIDDING: usize = 18;
pub const BUILD_FACTORY: usize = 19;
pub const BUILD_WAREHOUSE: usize = 20;
pub const START_AUCTION: usize = 21;
pub const OUTBID: usize = 22;
pub const BID_FRACTION: usize = 23;
pub const BID_CARGO: usize = 24;
pub const REPAY_CASH: usize = 25;
pub const LOAN_CASH: usize = 26;
pub const BUILD_CASH: usize = 27;
pub const BUY_SLOPE: usize = 28;
pub const PLACE_SLOPE: usize = 29;
pub const BANK_CASH_BID: usize = 30;
pub const BUY_CASH: usize = 31;

/// The parameters, with the simple opponent's behaviour as the defaults.
pub const PARAMS: [Param; 32] = [
    p("loan_broke", 110.0, 0.0, 200.0),
    p("loan_low", 0.0, 0.0, 200.0),
    p("repay", 105.0, 0.0, 200.0),
    p("sail_island", 100.0, 0.0, 200.0),
    p("produce", 90.0, 30.0, 200.0),
    p("load", 80.0, 0.0, 200.0),
    p("done_loading", 10.0, 0.0, 200.0),
    p("done_buying", 75.0, 0.0, 200.0),
    p("harbor_buy", 70.0, 0.0, 200.0),
    p("factory_buy", 65.0, 0.0, 200.0),
    p("buy", 70.0, 0.0, 200.0),
    p("sail_ocean", 55.0, 0.0, 200.0),
    p("sail_harbor", 40.0, 0.0, 200.0),
    p("sail_bank", 40.0, 0.0, 200.0),
    p("reprice", 5.0, 0.0, 200.0),
    p("place", 50.0, 0.0, 200.0),
    p("accept", 50.0, 0.0, 200.0),
    p("buy_out", 0.0, 0.0, 200.0),
    p("done_bidding", 90.0, 0.0, 200.0),
    p("build_factory", 0.0, 0.0, 200.0),
    p("build_warehouse", 0.0, 0.0, 200.0),
    p("start_auction", 1.0, 0.0, 200.0),
    p("outbid", 1.0, 0.0, 200.0),
    p("bid_fraction", 0.25, 0.0, 1.0),
    p("bid_cargo", 0.0, 0.0, 3.0),
    p("repay_cash", 25.0, 5.0, 80.0),
    p("loan_cash", 0.0, 0.0, 12.0),
    p("build_cash", 0.0, 0.0, 60.0),
    p("buy_slope", 1.0, 0.0, 12.0),
    p("place_slope", 1.0, 0.0, 12.0),
    p("bank_cash_bid", 1.0, 0.0, 1.0),
    p("buy_cash", 0.0, 0.0, 30.0),
];

/// Parameters tuned in the arena. Regenerate with `cargo run -p container-arena --release --bin tune`.
pub const TUNED: [f64; 32] = [
    110.000,
    0.000,
    11.410,
    91.998,
    30.000,
    36.942,
    11.250,
    12.870,
    70.000,
    33.449,
    70.000,
    55.000,
    82.784,
    0.000,
    12.818,
    134.854,
    61.399,
    0.000,
    90.000,
    0.000,
    0.000,
    91.357,
    1.000,
    0.150,
    0.000,
    25.000,
    0.000,
    0.000,
    1.000,
    2.784,
    1.000,
    3.857,
];

pub fn defaults() -> Vec<f64> {
    PARAMS.iter().map(|p| p.default).collect()
}

/// How a trader differs from the tuned Merchant: it prices its containers around one lot, and buys
/// only at prices it finds fair. Merchant takes anything at any price and sells at the cheapest lot,
/// so a net that trains against it learns that the dearest lot costs nothing. A person does neither.
#[derive(Clone, Copy, Debug)]
pub struct Trader {
    /// The most it pays in a factory district (a harbor lot costs one more for the same lot).
    pub max_price: u32,
    /// The lot it prefers to put its containers in.
    pub target_lot: usize,
}

impl Trader {
    /// A trader whose tastes follow from the seed: it pays up to $2, $3 or $4 and prices around $1 to $4.
    pub fn from_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed ^ 0x7ADE);
        Self {
            max_price: 2 + rng.below(3),
            target_lot: rng.below(4) as usize,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Merchant {
    rng: Rng,
    w: Vec<f64>,
    trader: Option<Trader>,
}

impl Merchant {
    /// The tuned opponent.
    pub fn new(seed: u64) -> Self {
        Self::with_params(seed, TUNED.to_vec())
    }

    pub fn with_params(seed: u64, w: Vec<f64>) -> Self {
        assert_eq!(w.len(), PARAMS.len());
        Self {
            rng: Rng::new(seed),
            w,
            trader: None,
        }
    }

    /// A Merchant that lays its containers around lot `target_lot` and buys only at lots that cost at
    /// most `max_price` in a factory district. With a low `target_lot` it is a cheap seller.
    pub fn seller(seed: u64, max_price: u32, target_lot: usize) -> Self {
        Self {
            trader: Some(Trader {
                max_price,
                target_lot,
            }),
            ..Self::new(seed)
        }
    }

    /// The tuned Merchant with a trader's prices.
    pub fn trader(seed: u64) -> Self {
        Self {
            trader: Some(Trader::from_seed(seed)),
            ..Self::new(seed)
        }
    }
}

impl Opponent for Merchant {
    fn choose(&mut self, obs: &Observation, legal: &[Action]) -> Action {
        let scores: Vec<f64> = legal.iter().map(|&a| score(&self.w, self.trader, obs, a)).collect();
        let best = scores.iter().cloned().fold(f64::MIN, f64::max);
        let top: Vec<Action> = legal
            .iter()
            .zip(&scores)
            .filter(|&(_, &s)| s >= best - 1e-9)
            .map(|(&a, _)| a)
            .collect();
        top[self.rng.below(top.len() as u32) as usize]
    }
}

fn score(w: &[f64], trader: Option<Trader>, obs: &Observation, action: Action) -> f64 {
    let me = &obs.players[obs.viewer];
    let cash = me.cash.unwrap_or(0);
    let loaded = me.ship.total() > 0;
    let at_main = obs.phase == PhaseView::Open(Phase::Main);
    let c = cash as f64;
    if let Some(t) = trader {
        let affordable = |lots: &[container_engine::color::ColorCounts], first_price: u32| {
            lots.iter().enumerate().any(|(lot, n)| {
                let price = first_price + lot as u32;
                !n.is_empty() && price <= t.max_price + first_price - 1 && price <= cash
            })
        };
        match action {
            Action::FactoryPurchase(p) if !affordable(&obs.players[p].factory_lots, 1) => return -1.0,
            Action::Sail(ShipLocation::Harbor(p)) if !affordable(&obs.players[p].harbor_lots, 2) => return -1.0,
            Action::Buy { lot, .. } => {
                if let PhaseView::Open(Phase::Buy(b)) = obs.phase {
                    let first = if b.district == District::Factory { 1 } else { 2 };
                    if first + lot as u32 > t.max_price + first - 1 {
                        return 1.0;
                    }
                }
            }
            Action::Place(lot) => return w[PLACE] - 8.0 * (lot as f64 - t.target_lot as f64).abs(),
            _ => {}
        }
    }
    match action {
        Action::RequestLoan if at_main && cash == 0 => w[LOAN_BROKE],
        Action::RequestLoan if at_main && c <= w[LOAN_CASH] => w[LOAN_LOW],
        Action::RepayLoan if c >= w[REPAY_CASH] => w[REPAY],
        Action::Sail(ShipLocation::Island) if loaded => w[SAIL_ISLAND],
        Action::Produce => w[PRODUCE],
        Action::Load(_) => w[LOAD],
        Action::DoneLoading => w[DONE_LOADING],
        Action::DoneBuying => w[DONE_BUYING],
        Action::HarborPurchase => w[HARBOR_BUY],
        Action::FactoryPurchase(_) => w[FACTORY_BUY],
        Action::Buy { lot, .. } if c >= w[BUY_CASH] => w[BUY] - w[BUY_SLOPE] * lot as f64,
        Action::Sail(ShipLocation::Ocean) if loaded => w[SAIL_OCEAN],
        Action::Sail(ShipLocation::Harbor(_)) => w[SAIL_HARBOR],
        Action::Sail(ShipLocation::Bank) => w[SAIL_BANK],
        Action::Sail(_) => 20.0,
        Action::Reprice(_) => w[REPRICE],
        Action::Place(lot) => w[PLACE] - w[PLACE_SLOPE] * lot as f64,
        Action::ProduceColor(_) | Action::Seize { .. } | Action::Deal(_) => 50.0,
        Action::Accept(_) => w[ACCEPT],
        Action::BuyOut => w[BUY_OUT],
        Action::Bid(amount) => {
            let target = c * w[BID_FRACTION] + w[BID_CARGO] * me.ship.total() as f64;
            1_000.0 - (amount as f64 - target).abs().min(900.0)
        }
        Action::BidCash(amount) => 100.0 - (amount as f64 * w[BANK_CASH_BID]).min(100.0),
        Action::DoneBidding => w[DONE_BIDDING],
        Action::Build(Building::Factory(_)) if c >= w[BUILD_CASH] => w[BUILD_FACTORY],
        Action::Build(Building::Warehouse) if c >= w[BUILD_CASH] => w[BUILD_WAREHOUSE],
        Action::StartAuction { .. } => w[START_AUCTION],
        Action::Outbid(_) => w[OUTBID],
        Action::BidContainer { .. } => 1.0,
        _ => 0.0,
    }
}
